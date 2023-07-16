use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use thiserror::Error;
use validator::Validate;

// 汎用的なエラーメッセージをここに集結させる。
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected Error: {0}")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(min = 100, message = "Over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}

// Todo そのものの構造体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}
// Todo　リポジトリインターフェース
#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo>;
    async fn all(&self) -> Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo>;
    async fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDB {
    pool: PgPool,
}

impl TodoRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDB { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDB {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        dbg!(payload.text.clone());
        let todo = sqlx::query_as::<_, Todo>(
            r#"
insert into todos (text, completed)
values ($1, false)
returning *;
        "#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }
    async fn find(&self, id: i32) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
select * from todos where id=$1
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;
        Ok(todo)
    }
    async fn all(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
select * from todos
order by id desc;
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
        let old_todo = self.find(id).await?;
        let todo = sqlx::query_as::<_, Todo>(
            r#"
update todos set text=$1, completed=$2
where id=$3
returning *
        "#,
        )
        .bind(payload.text.unwrap_or(old_todo.text))
        .bind(payload.completed.unwrap_or(old_todo.completed))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }
    async fn delete(&self, id: i32) -> Result<()> {
        let _ = sqlx::query_as::<_, Todo>(
            r#"
delete from todos where id=$1
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        });

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::handler;

    use super::*;
    use actix_web::{
        http::header::ContentType,
        test,
        web::{self},
        App,
    };
    use dotenv::dotenv;
    use pretty_assertions::assert_eq;
    use sqlx::PgPool;
    use tracing::{debug, instrument};
    use tracing_subscriber::EnvFilter;

    use std::{env, sync::Once};
    static INIT: Once = Once::new();
    fn initialize_tracing() {
        INIT.call_once(|| {
            let log_level = env::var("RUST_LOG").unwrap_or("debug".into());
            env::set_var("RUST_LOG", log_level);
            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        });
    }
    // #[actix_web::test]
    #[instrument(ret)]
    async fn crud_scenario() {
        initialize_tracing();

        dotenv().ok();

        // DB接続
        let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");

        debug!("start connect database...");
        let pool = PgPool::connect(database_url)
            .await
            .unwrap_or_else(|_| panic!("fail coonect database, usl is [{database_url}]"));

        //初期化
        let repository = web::Data::new(TodoRepositoryForDB::new(pool));

        let app =
            test::init_service(App::new().app_data(repository).configure(handler::config)).await;

        let expected = Todo::new(1, "[crud_scenario] text".to_string());

        let actual = CreateTodo {
            text: "[crud_scenario] text".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/todos")
            .insert_header(ContentType::json())
            .set_json(&actual)
            .to_request();

        let resp: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expected, resp);
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use anyhow::{Context, Result};
    use async_trait::async_trait;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    impl CreateTodo {
        pub fn new(text: String) -> Self {
            Self { text }
        }
    }
    type TodoDatas = HashMap<i32, Todo>;

    //メモリ上にTodoリストを保存するための構造体
    #[derive(Debug, Clone)]
    pub struct TodoRepositoryForMemory {
        store: Arc<RwLock<TodoDatas>>,
    }

    impl TodoRepositoryForMemory {
        pub fn new() -> Self {
            TodoRepositoryForMemory {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
            self.store.read().unwrap()
        }
    }

    impl Default for TodoRepositoryForMemory {
        fn default() -> Self {
            Self::new()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryForMemory {
        async fn create(&self, payload: CreateTodo) -> Result<Todo> {
            let mut store = self.write_store_ref();
            // Todo: 例えば、idが2のtodoが1つしかない場合は、いくらcreateしてもtodoが作れないバグがある。
            let id = (store.len() + 1) as i32;
            let todo = Todo::new(id, payload.text);
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> Result<Todo> {
            let store = self.read_store_ref();
            let todo = store
                .get(&id)
                .cloned()
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self) -> Result<Vec<Todo>> {
            let store = self.read_store_ref();
            Ok(Vec::from_iter(store.values().cloned()))
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
            let mut store = self.write_store_ref();
            let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
            let text = payload.text.unwrap_or(todo.text.clone());
            let completed = payload.completed.unwrap_or(todo.completed);
            let todo = Todo {
                id,
                text,
                completed,
            };
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> Result<()> {
            // Todo: Create側のバグがあるので、そのバグをここで防ぐかどうか。
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }

    mod test {
        use super::*;

        #[actix_web::test]
        async fn todo_crud_scenario() {
            let text = "todo test".to_string();
            let id = 1;
            let expected = Todo::new(id, text.clone());

            //create : Todoを作成
            let repository = TodoRepositoryForMemory::new();
            let todo = repository
                .create(CreateTodo { text })
                .await
                .expect("failed create todo.");
            assert_eq!(expected, todo);

            //find　：Todo idを取得
            let todo = repository.find(todo.id).await.unwrap();
            assert_eq!(expected, todo);

            //all　全てのTodoを取得
            let todo = repository.all().await.expect("failed get all todo.");
            assert_eq!(vec![expected], todo);

            // update　： Todoを更新
            let text = "update todo text".to_string();
            let todo = repository
                .update(
                    1,
                    UpdateTodo {
                        text: Some(text.clone()),
                        completed: Some(true),
                    },
                )
                .await
                .expect("failed update todo.");

            assert_eq!(
                Todo {
                    id,
                    text,
                    completed: true
                },
                todo
            );

            // delete　：Todoを削除
            let res = repository.delete(id).await;
            assert!(res.is_ok())
        }
    }
}
