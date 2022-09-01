use reqwest::{Client, Method, Response, Result};
use serde::Serialize;

pub enum Auth {
    None,
    Basic { username: String, password: String },
    Bearer { token: String },
    Header { key: String, value: String },
}

#[doc(hidden)]
pub enum Body<'a, T: Serialize + ?Sized = ()> {
    None,
    Json(&'a T),
    Form(&'a T),
}

#[async_trait::async_trait(?Send)]
pub trait Api {
    fn client(&self) -> &Client;
    fn auth(&self) -> &Auth;

    #[inline]
    async fn request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        url: &str,
        body: Body<'_, T>,
    ) -> Result<Response> {
        let request = self.client().request(method, url);
        let request = match self.auth() {
            Auth::None => request,
            Auth::Basic { username, password } => request.basic_auth(username, Some(password)),
            Auth::Bearer { token } => request.bearer_auth(token),
            Auth::Header { key, value } => request.header(key, value),
        };
        let request = match body {
            Body::None => request,
            Body::Json(body) => request.json(body),
            Body::Form(body) => request.form(body),
        };
        request.send().await
    }
}

#[macro_export]
macro_rules! api {
    () => {};

    ($vis:vis struct $ident:ident) => {
        $vis struct $ident {
            client: ::reqwest::Client,
            auth: $crate::Auth
        }

        impl $ident {
            pub fn new(auth: $crate::Auth) -> Self {
                Self {
                    client: ::reqwest::Client::new(),
                    auth
                }
            }
        }

        impl $crate::Api for $ident {
            fn client(&self) -> &::reqwest::Client {
                &self.client
            }

            fn auth(&self) -> &$crate::Auth {
                &self.auth
            }
        }
    };

    ($vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty),*) -> StatusCode { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Json(request)).await.map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty),*) -> String { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Json(request)).await?.text().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty),*) -> Bytes { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Json(request)).await?.bytes().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty),*) -> Json<$res:ty> { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Json(request)).await?.json().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty),*) -> StatusCode { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Form(request)).await.map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty),*) -> String { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Form(request)).await?.text().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty),*) -> Bytes { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Form(request)).await?.bytes().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty),*) -> Json<$res:ty> { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.request(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::Form(request)).await?.json().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> StatusCode { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.request::<()>(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::None).await.map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> String { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.request::<()>(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::None).await?.text().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> Bytes { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.request::<()>(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::None).await?.bytes().await
        }
        api!($($rest)*);
    };

    ($vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> Json<$res:ty> { $method:tt $url:literal } $($rest:tt)*) => {
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.request::<()>(::reqwest::Method::$method, format!($url).as_str(), $crate::Body::None).await?.json().await
        }
        api!($($rest)*);
    };
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use super::{api, Api, Auth};
    use example::*;

    mod example {
        use super::api;

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

    #[test]
    fn json_placeholder() {
        tokio_test::block_on(async {
            let api = JsonPlaceholder::new(Auth::None);

            let all_todos = api.todos().await.unwrap();
            let todo_1 = api.todo(1).await.unwrap();
            assert_eq!(&all_todos[0], &todo_1);

            let new_todo = api
                .create_todo(&CreateTodo {
                    user_id: 1,
                    title: "test".to_string(),
                    completed: false,
                })
                .await
                .unwrap();
            assert_eq!(new_todo.id, (all_todos.len() + 1) as u32);

            let replaced_todo = api
                .replace_todo(
                    &Todo {
                        title: "test".to_string(),
                        completed: true,
                        ..todo_1
                    },
                    1,
                )
                .await
                .unwrap();
            assert_eq!(replaced_todo.title, "test");
            assert!(replaced_todo.completed);

            let updated_todo = api
                .update_todo(
                    &UpdateTodo {
                        title: Some("test".to_string()),
                        completed: Some(true),
                        ..Default::default()
                    },
                    1,
                )
                .await
                .unwrap();
            assert_eq!(updated_todo.title, "test");
            assert!(updated_todo.completed);

            assert!(api.delete_todo(1).await.unwrap().is_success());
        });
    }
}
