use super::{UserForm, UserSignupForm};
use crate::entities::User;
use crate::middleware::{Claims, AuthBody, KEY_ENCODING};
use crate::model::database::{add_user, verify_user, DB_NAME, USERS_COLL};
use axum::{
    extract::{State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{StatusCode},
    response::IntoResponse,
    Form, Json,
};
use chrono::{prelude::*, Duration};
// use axum_database_sessions::{AxumRedisPool, AxumSession};
// use axum_sessions_auth::AuthSession;
use jsonwebtoken::{encode, Header};
use redis::{Client, Commands};

pub async fn signup(
    Form(input): Form<UserSignupForm>,
    State(client): State<Client>,
) -> impl IntoResponse {
    let user: User = input.into();

    // let id = match verify_user(&client, &user).await {
    //     Ok(r) => {
    //         if r {
    //             user.name
    //         } else {
    //             return (StatusCode::INTERNAL_SERVER_ERROR, "no such user").into_response();
    //         }
    //     }
    //     Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response(),
    // };
    // session.insert("user_id", &id)?;
    // session.renew();

    (StatusCode::OK, "ok").into_response()
}

pub async fn login(Form(input): Form<UserForm>) -> impl IntoResponse {
    let user: User = input.clone().into();

    // check if the user is signed-up;

    let claims = Claims {
        id: 0,
        exp: (Local::now() + Duration::minutes(input.remember_mins)).timestamp() as u64,
        iat: Local::now().timestamp() as u64,
        username: user.name,
    };

    // Create the authorization token
    // TODO: need to change this unwrap
    let token = encode(&Header::default(), &claims, &KEY_ENCODING).unwrap();

    // Send the authorized token
    Json(AuthBody::new(token))
}

pub async fn logout(
    State(state): State<Client>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    // added this jwt to the redis blacklist.
    // auth.token();
    let mut con = state.get_connection().unwrap();
    con.set::<&str, i32, ()>(auth.token(), 42).unwrap();
    (StatusCode::OK, "ok").into_response()
}
