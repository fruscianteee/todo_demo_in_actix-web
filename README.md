# ToDo Demo in Actix-Web

## Overview
This is a simple ToDo application created using Actix-Web framework.
The application provides CRUD operations for managing tasks, along with features such as labeling and validation.
When launched, the web interface can be accessed at `localhost:3000`.

## Features
- Create, Read, Update, and Delete (CRUD) operations for managing tasks
- Task labeling for better organization
- Validation of task data
- Web interface accessible at `localhost:3000`

## Dependencies
The following are the main crates used in this application:
- `actix-web`: A powerful, pragmatic, and extremely fast web framework for Rust.
- `anyhow`: Provides flexible error handling with a simple `Error` type.
- `thiserror`: A lightweight crate for easily defining custom error types.
- `tracing`: A framework for instrumenting Rust programs with context-aware, structured logging and diagnostics.
- `serde`: A powerful framework for serializing and deserializing Rust data structures.
- `validator`: A crate for data validation, including email, URL, and custom validations.
- `askama`: A type-safe, compiled templating engine for Rust.
- `sqlx`: Async, pure Rust SQL toolkit and ORM.

## Usage
1. Clone the repository: `git clone https://github.com/fruscianteee/todo_demo_in_actix-web.git`
2. Change into the project directory: `cd todo_demo_in_actix-web`
3. Build and run the application: `cargo run`
4. Access the web interface in your browser at `localhost:3000`

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
