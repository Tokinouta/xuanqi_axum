use anyhow::Result;
use axum::{async_trait, routing::post, Router};
use axum_sessions_auth::{Authentication, HasPermission};
use chrono::{DateTime, Local};

use redis::Client as redisClient;
use serde::{Deserialize, Serialize};

use self::auth::{login, logout};

pub mod auth;

static CURRENT_USER_ID: i64 = 2;

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    auth: String,
    remember_mins: i32,
    time: DateTime<Local>,
}

#[derive(Clone, Debug)]
pub struct UserInSession {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}

// This is only used if you want to use Token based Authentication checks
#[async_trait]
impl HasPermission<redisClient> for UserInSession {
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
impl Authentication<UserInSession, i64, redisClient> for UserInSession {
    async fn load_user(userid: i64, _pool: Option<&redisClient>) -> Result<UserInSession> {
        Ok(UserInSession {
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
