# API Client &emsp; [![MIT licensed][mit-badge]][mit-url] [![Build Status][actions-badge]][actions-url] [![Documentation][docs-badge]][docs-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/JohnPeel/api-client/blob/master/LICENSE
[actions-badge]: https://github.com/JohnPeel/api-client/workflows/CI/badge.svg
[actions-url]: https://github.com/JohnPeel/api-client/actions?query=workflow%3ACI+branch%3Amaster
[docs-badge]: https://img.shields.io/docsrs/api-client
[docs-url]: https://docs.rs/api-client/latest/api_client

This project provides a macro for quickly creating REST api client structs in Rust.

## Adding to project

```toml
[dependencies]
api-client = "0.1"
```

## Example

```rust
mod api {
    use api_client::{api, Api, Auth};

    pub use models::*;

    mod models {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
        pub struct Todo {
            #[serde(rename = "userId")]
            pub user_id: u32,
            pub id: u32,
            pub title: String,
            pub completed: bool,
        }

        #[derive(Debug, Serialize)]
        pub struct CreateTodo {
            #[serde(rename = "userId")]
            pub user_id: u32,
            pub title: String,
            pub completed: bool,
        }

        #[derive(Debug, Default, Serialize)]
        pub struct UpdateTodo {
            #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
            pub user_id: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub completed: Option<bool>,
        }
    }

    api!(pub struct JsonPlaceholder);

    const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

    impl JsonPlaceholder {
        pub fn new() -> Self {
            Api::new(Auth::None)
        }

        api! {
            pub fn todos() -> Json<Vec<Todo>> {
                GET "{BASE_URL}/todos"
            }

            pub fn todo(id: u32) -> Json<Todo> {
                GET "{BASE_URL}/todos/{id}"
            }

            pub fn create_todo(request: Json<CreateTodo>) -> Json<Todo> {
                POST "{BASE_URL}/todos"
            }

            pub fn replace_todo(request: Json<Todo>, id: u32) -> Json<Todo> {
                PUT "{BASE_URL}/todos/{id}"
            }

            pub fn update_todo(request: Json<UpdateTodo>, id: u32) -> Json<Todo> {
                PATCH "{BASE_URL}/todos/{id}"
            }

            pub fn delete_todo(id: u32) -> StatusCode {
                DELETE "{BASE_URL}/todos/{id}"
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let json_placeholder = api::JsonPlaceholder::new();

    let todo_1 = json_placeholder.todo(1).await?;
    println!("{:?}", todo_1);

    Ok(())
}
```