use axum::{Router, Form, extract::Extension, response::IntoResponse, routing::post,http::StatusCode};
use chrono::{DateTime, Local};
// use actix_session::Session;
// use actix_web::{
//     post, web, HttpResponse, Result,
// };
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{add_user, verify_user, DB_NAME, USERS_COLL},
    Repo, User,
};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    auth: String,
    remember_mins: i32,
    time: DateTime<Local>,
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

pub async fn login(Form(input): Form<UserForm>,  Extension(client): Extension<Client>) -> impl IntoResponse {
    let user = User::new(input.username, input.auth);

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}


pub fn router() -> Router {
    // let category_router = Router::new();
    // let topic_router = Router::new();
    Router::new()
        .nest("/login", post(login))
        // .nest("/topic", topic_router)
}
