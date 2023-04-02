#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]

use reqwest::{Client, RequestBuilder, Result};

#[doc(hidden)]
pub use reqwest;

/// Used internally to the api! macro.
#[doc(hidden)]
pub enum Body<'a, T: ?Sized = ()> {
    /// No body.
    None,
    /// JSON body.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    Json(&'a T),
    /// Form body.
    Form(&'a T),
    #[cfg(feature = "multipart")]
    #[cfg_attr(docsrs, doc(cfg(feature = "multipart")))]
    /// Multipart body.
    Multipart(reqwest::multipart::Form),
}

/// The main API trait.
///
/// If you need custom behavior, such as authentication, you should implement this trait on your custom struct. See the [Api::pre_request] method for more details.
///
/// Otherwise, you can use the [api] macro to generate a struct with a proper implementation of this trait.
#[async_trait::async_trait(?Send)]
pub trait Api {
    /// Returns a reference to a reqwest Client to create requests.
    fn client(&self) -> &Client;

    /// You can use this method to modify the request before sending it.
    ///
    /// Some good examples of usage are:
    ///  - Authentication
    ///  - Custom headers (can also be done with a method on Client)
    ///
    /// # Authentication
    /// ```rust
    /// use api_client::{api, Api};
    /// use reqwest::{Client, RequestBuilder};
    ///
    /// struct ExampleApi {
    ///     client: Client,
    ///     username: String,
    ///     password: String
    /// }
    ///
    /// impl Api for ExampleApi {
    ///     fn client(&self) -> &Client {
    ///         &self.client
    ///     }
    ///
    ///     fn pre_request(&self, request: RequestBuilder) -> reqwest::Result<RequestBuilder> {
    ///         Ok(request.basic_auth(&self.username, Some(&self.password)))
    ///     }
    /// }
    ///
    /// impl ExampleApi {
    ///     api! {
    ///         fn example() -> String {
    ///            GET "https://example.com"
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    fn pre_request(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        Ok(request)
    }

    /// Used internally in the api! macro. Mostly for ergonmics.
    ///
    /// # Usage
    /// ```rust
    /// # use api_client::{api, Api};
    ///
    /// api!(pub struct Example);
    ///
    /// fn main() {
    ///     let example = Example::new();
    /// }
    /// ```
    #[doc(hidden)]
    #[inline]
    #[must_use]
    fn new() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

/// Magic macro for API structs.
///
/// # Simple Usage (auto generated struct)
/// ```rust
/// use api_client::{api, Api};
/// use reqwest::{Client, RequestBuilder};
///
/// api!(pub struct ExampleApi);
///
/// impl ExampleApi {
///     api! {
///         fn example() -> String {
///            GET "https://example.com"
///         }
///     }
/// }
/// ```
///
/// # Advanced Usage (manually created struct and [Api] implementation)
/// ```rust
/// use api_client::{api, Api};
/// use reqwest::{Client, RequestBuilder};
///
/// struct ExampleApi {
///     client: Client,
///     username: String,
///     password: String
/// }
///
/// impl Api for ExampleApi {
///     fn client(&self) -> &Client {
///         &self.client
///     }
///
///     fn pre_request(&self, request: RequestBuilder) -> reqwest::Result<RequestBuilder> {
///         Ok(request.basic_auth(&self.username, Some(&self.password)))
///     }
/// }
///
/// impl ExampleApi {
///     api! {
///         fn example() -> String {
///            GET "https://example.com"
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! api {
    () => {};

    ($(#[$attr:meta])* $vis:vis struct $ident:ident) => {
        $(#[$attr])*
        $vis struct $ident(::reqwest::Client);

        impl $crate::Api for $ident {
            fn client(&self) -> &::reqwest::Client {
                &self.0
            }

            fn new() -> Self where Self: Sized {
                $ident(::reqwest::Client::new())
            }
        }
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty)*) -> StatusCode { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .json(request)
                .send()
                .await
                .map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty)*) -> String { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .json(request)
                .send()
                .await?
                .text()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty)*) -> Bytes { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .json(request)
                .send()
                .await?
                .bytes()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Json<$req:ty>$(, $name:ident: $ty:ty)*) -> Json<$res:ty> { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .json(request)
                .send()
                .await?
                .json()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty)*) -> StatusCode { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .form(request)
                .send()
                .await
                .map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty)*) -> String { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .form(request)
                .send()
                .await?
                .text()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty)*) -> Bytes { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .form(request)
                .send()
                .await?
                .bytes()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident(request: Form<$req:ty>$(, $name:ident: $ty:ty)*) -> Json<$res:ty> { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, request: &$req, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .form(request)
                .send()
                .await?
                .json()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> StatusCode { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<::reqwest::StatusCode> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .send()
                .await
                .map(|res| res.status())
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> String { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<String> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .send()
                .await?
                .text()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> Bytes { $method:tt $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<::bytes::Bytes> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .send()
                .await?
                .bytes()
                .await
        }
        api!($($rest)*);
    };

    ($(#[$attr:meta])* $vis:vis fn $ident:ident($($name:ident: $ty:ty),*) -> Json<$res:ty> { $method:ident $url:literal $($headername:ident: $headervalue:expr)* } $($rest:tt)*) => {
        $(#[$attr])*
        #[inline]
        $vis async fn $ident(&self, $($name: $ty),*) -> ::reqwest::Result<$res> {
            use $crate::Api as _;
            self.pre_request(self.client().request($crate::reqwest::Method::$method, format!($url).as_str()))?
                $(.header($crate::reqwest::header::$headername, format!($headervalue).as_str()))*
                .send()
                .await?
                .json()
                .await
        }
        api!($($rest)*);
    };
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use example::{CreateTodo, JsonPlaceholder, Todo, UpdateTodo};

    use self::headers::HeaderTest;

    mod example {
        use crate::{api, Api};

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
                Api::new()
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

    #[tokio::test]
    async fn json_placeholder() {
        let api = JsonPlaceholder::new();

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
        assert_eq!(new_todo.id as usize, all_todos.len() + 1);

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
    }

    mod headers {
        use crate::{api, Api};

        api!(pub struct HeaderTest);

        const BASE_URL: &str = "https://ifconfig.me";

        impl HeaderTest {
            pub fn new() -> Self {
                Api::new()
            }

            api! {
                pub fn get_ua(ua: &str) -> String {
                    GET "{BASE_URL}/ua"
                    USER_AGENT: "{ua}"
                }
            }
        }
    }

    #[tokio::test]
    async fn example_header() {
        let api = HeaderTest::new();

        assert_eq!(
            api.get_ua("Api-client 0.1").await.unwrap(),
            "Api-client 0.1"
        );
    }
}
