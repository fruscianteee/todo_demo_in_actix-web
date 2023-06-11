use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;
use validator::Validate;

// 汎用的なエラーメッセージをここに集結させる。
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

// Todo そのものの構造体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: String,
}

#[cfg(test)]
impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(min = 100, message = "Over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
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

// Todo　リポジトリインターフェース
#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo>;
    async fn all(&self) -> Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo>;
    async fn delete(&self, id: i32) -> Result<()>;
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

#[cfg(test)]
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
