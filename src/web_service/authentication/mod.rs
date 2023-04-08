// use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    http::{
        self,
        header::{HeaderMap, HeaderValue},
        request::Parts,
        StatusCode,
    },
    response::IntoResponse,
    routing::get,
    RequestPartsExt, Router,
};
// use axum_extra::extract::cookie::Cookie;
// use axum_sessions_auth::{Authentication, HasPermission};
use chrono::{DateTime, Local};

use redis::Client as redisClient;
use serde::{Deserialize, Serialize};

// use async_redis_session::RedisSessionStore;
// use axum_sessions::{
//     async_session::{self, MemoryStore, Session, SessionStore},
//     extractors::WritableSession,
//     SessionLayer,
// };
// use sqlx::types::Uuid;

use crate::entities::User;

// use self::auth::{login, logout};

pub mod auth;

// static CURRENT_USER_ID: i64 = 2;
const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Clone, Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    auth: String,
    remember_mins: i64,
    time: DateTime<Local>,
}

impl Into<User> for UserForm {
    fn into(self) -> User {
        User {
            name: self.username,
            auth: self.auth,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserSignupForm {
    username: String,
    auth: String,
}

impl Into<User> for UserSignupForm {
    fn into(self) -> User {
        User {
            name: self.username,
            auth: self.auth,
        }
    }
}

// struct FreshUserId {
//     pub user_id: UserId,
//     pub cookie: HeaderValue,
// }

// enum UserIdFromSession {
//     FoundUserId(UserId),
//     CreatedFreshUserId(FreshUserId),
// }

// #[async_trait]
// impl<S> FromRequestParts<S> for UserIdFromSession
// where
//     MemoryStore: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let store = RedisSessionStore::from_ref(state);

//         let cookie: Option<TypedHeader<Cookie>> = parts.extract().await.unwrap();

//         let session_cookie = cookie
//             .as_ref()
//             .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

//         // return the new created session cookie for client
//         if session_cookie.is_none() {
//             let user_id = UserId::new();
//             let mut session = Session::new();
//             session.insert("user_id", user_id).unwrap();
//             let cookie = store.store_session(session).await.unwrap().unwrap();
//             return Ok(Self::CreatedFreshUserId(FreshUserId {
//                 user_id,
//                 cookie: HeaderValue::from_str(
//                     format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str(),
//                 )
//                 .unwrap(),
//             }));
//         }

//         tracing::debug!(
//             "UserIdFromSession: got session cookie from user agent, {}={}",
//             AXUM_SESSION_COOKIE_NAME,
//             session_cookie.unwrap()
//         );
//         // continue to decode the session cookie
//         let user_id = if let Some(session) = store
//             .load_session(session_cookie.unwrap().to_owned())
//             .await
//             .unwrap()
//         {
//             if let Some(user_id) = session.get::<UserId>("user_id") {
//                 tracing::debug!(
//                     "UserIdFromSession: session decoded success, user_id={:?}",
//                     user_id
//                 );
//                 user_id
//             } else {
//                 return Err((
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     "No `user_id` found in session",
//                 ));
//             }
//         } else {
//             tracing::debug!(
//                 "UserIdFromSession: err session not exists in store, {}={}",
//                 AXUM_SESSION_COOKIE_NAME,
//                 session_cookie.unwrap()
//             );
//             return Err((StatusCode::BAD_REQUEST, "No session found for cookie"));
//         };

//         Ok(Self::FoundUserId(user_id))
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// struct UserId(Uuid);

// impl UserId {
//     fn new() -> Self {
//         Self(Uuid::new_v4())
//     }
// }
// fn a() {
//     let store = RedisSessionStore::new("redis://127.0.0.1/").unwrap();

//     let store = async_session::MemoryStore::new();
//     let secret = b"..."; // MUST be at least 64 bytes!
//     let session_layer = SessionLayer::new(store, secret);

//     async fn handler(mut session: WritableSession) {
//         session
//             .insert("foo", 42)
//             .expect("Could not store the answer.");
//     }
// }

pub fn router() -> Router {
    // let category_router = Router::new();
    // let topic_router = Router::new();
    Router::new()
    // .route("/login", post(login))
    // .route("/logout", post(logout))
    // .nest("/topic", topic_router)
}

// #[post("/signup")]
// async fn signup(client: web::Data<Client>, user_id: web::Form<User>) -> HttpResponse {
//     // remember identity
//     let user = user_id.into_inner();
//     println!("{}", user.name);
//     match add_user(&client, &user).await {
//         Ok(_) => HttpResponse::Ok().body("user added"),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[post("/login")]
// async fn login(
//     client: web::Data<Client>,
//     user_id: web::Form<User>,
//     session: Session,
// ) -> Result<HttpResponse> {
//     let user = user_id.into_inner();
//     let id = match verify_user(&client, &user).await {
//         Ok(r) => {
//             if r {
//                 user.name
//             } else {
//                 return Ok(HttpResponse::InternalServerError().body("no such user"));
//             }
//         }
//         Err(_) => return Ok(HttpResponse::InternalServerError().body("database error")),
//     };
//     session.insert("user_id", &id)?;
//     session.renew();

//     let counter: i32 = session
//         .get::<i32>("counter")
//         .unwrap_or(Some(0))
//         .unwrap_or(0);

//     Ok(HttpResponse::Ok().json(IndexResponse {
//         user_id: Some(id),
//         counter,
//     }))
// }

// #[post("/logout")]
// async fn logout(session: Session) -> Result<String> {
//     let id: Option<String> = session.get("user_id")?;
//     if let Some(x) = id {
//         session.purge();
//         Ok(format!("Logged out: {}", x))
//     } else {
//         Ok("Could not log out anonymous user".into())
//     }
// }
