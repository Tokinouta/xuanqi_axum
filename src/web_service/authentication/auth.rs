use super::{UserForm, UserSignupForm};
use crate::database::{add_user, verify_user};
use crate::entities::User;
use crate::middleware::{AuthBody, Claims, KEY_ENCODING};
use axum::{
    extract::{State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use chrono::{prelude::*, Duration};
// use axum_database_sessions::{AxumRedisPool, AxumSession};
// use axum_sessions_auth::AuthSession;
use jsonwebtoken::{encode, Header};
use redis::{Client, Commands};
use sqlx::PgPool;

pub async fn signup(
    Form(input): Form<UserSignupForm>,
    State(client): State<PgPool>,
) -> impl IntoResponse {
    let user: User = input.into();

    match add_user(&client, &user).await {
        Ok(_) => (StatusCode::OK, "ok").into_response(),
        Err(_) => todo!(),
    }
}

pub async fn login(Form(input): Form<UserForm>, State(client): State<PgPool>) -> impl IntoResponse {
    let user: User = input.clone().into();

    // check if the user is signed-up;
    match verify_user(&client, &user).await {
        Ok(r) => {
            if !r {
                return (StatusCode::INTERNAL_SERVER_ERROR, "no such user").into_response();
            }
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response(),
    };

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
    Json(AuthBody::new(token)).into_response()
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
