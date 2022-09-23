use anyhow::Result;
use axum::{
    async_trait,
    extract::{Extension, Path},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Form, Router,
};
use chrono::{DateTime, Local};
// use actix_session::Session;
// use actix_web::{
//     post, web, HttpResponse, Result,
// };
use axum_database_sessions::{
    AxumPgPool, AxumRedisPool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use axum_sessions_auth::{
    Auth, AuthSession, AuthSessionLayer, Authentication, HasPermission, Rights,
};
use mongodb::Client;
use redis::Client as redisClient;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{add_user, verify_user, DB_NAME, USERS_COLL},
    Repo,
};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    auth: String,
    remember_mins: i32,
    time: DateTime<Local>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}

pub async fn login1(
    Form(input): Form<UserForm>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    let user = crate::model::User::new(input.username, input.auth);

    let id = match verify_user(&client, &user).await {
        Ok(r) => {
            if r {
                user.name
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, "no such user").into_response();
            }
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response(),
    };
    // session.insert("user_id", &id)?;
    // session.renew();

    // let counter: i32 = session
    //     .get::<i32>("counter")
    //     .unwrap_or(Some(0))
    //     .unwrap_or(0);

    (StatusCode::OK, "ok").into_response()
}

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct IndexResponse {
//     user_id: Option<String>,
//     counter: i32,
// }

// This is only used if you want to use Token based Authentication checks
#[async_trait]
impl HasPermission<redisClient> for User {
    async fn has(&self, _perm: &str, _pool: &Option<&redisClient>) -> bool {
        false
        // match perm {
        //     "Token::UseAdmin" => true,
        //     "Token::ModifyUser" => true,
        //     _ => false,
        // }
    }
}

#[async_trait]
impl Authentication<User, i64, redisClient> for User {
    async fn load_user(userid: i64, _pool: Option<&redisClient>) -> Result<User> {
        Ok(User {
            id: userid,
            anonymous: true,
            username: "Guest".to_string(),
        })
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

// We can get the Method to compare with what Methods we allow. Useful if this supports multiple methods.
// When called auth is loaded in the background for you.
// TODO add remaining time support
async fn login(
    Form(input): Form<UserForm>,
    // session: AxumSession<AxumRedisPool>,
    auth: AuthSession<User, i64, AxumRedisPool, redisClient>,
) -> impl IntoResponse {
    if input.remember_mins != 3 && input.remember_mins != 10080 {
        return (StatusCode::OK, "Error: wrong remaining time").into_response();
    }

    let mut count: usize = auth.session.get("count").await.unwrap_or(0);
    count += 1;

    // Session is Also included with Auth so no need to require it in the function arguments if your using
    // AuthSession.
    auth.session.set("count", count).await;
    if let Some(_) = auth.current_user {
        // if !Auth::<User, i64, redisClient>::build([Method::GET], false)
        //     .requires(Rights::any([
        //         Rights::permission("Token::UseAdmin"),
        //         Rights::permission("Token::ModifyPerms"),
        //     ]))
        //     .validate(&cur_user, &method, None)
        //     .await
        // {
        //     return format!("No Permissions! for {}", cur_user.username);
        // }

        if !auth.is_authenticated() {
            // Set the user ID of the User to the Session so it can be Auto Loaded the next load or redirect
            auth.login_user(input.auth.parse().unwrap_or(0)).await;
            return (StatusCode::OK, "ok").into_response();
        } else {
            // If the user is loaded and is Authenticated then we can use it.
            return (StatusCode::OK, "already_logged_in").into_response();
        }

        // format!("{}-{}", username, count)
    } else {
        if !auth.is_authenticated() {
            // Set the user ID of the User to the Session so it can be Auto Loaded the next load or redirect
            auth.login_user(input.auth.parse().unwrap_or(0)).await;
            // Set the session to be long term. Good for Remember me type instances.
            auth.remember_user(true).await;
            // Redirect here after login if we did indeed login.
            return (StatusCode::OK, "ok").into_response();
        }
        return (StatusCode::OK, "error").into_response();
    }
}

async fn logout(
    session: AxumSession<AxumRedisPool>,
    auth: AuthSession<User, i64, AxumRedisPool, redisClient>,
) -> impl IntoResponse {
    if auth.is_authenticated() {
        // Set the user ID of the User to the Session so it can be Auto Loaded the next load or redirect
        auth.logout_user().await;
        (StatusCode::OK, "ok").into_response()
    } else {
        (StatusCode::OK, "no-need").into_response()
    }
}

pub fn router() -> Router {
    // let category_router = Router::new();
    // let topic_router = Router::new();
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
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
