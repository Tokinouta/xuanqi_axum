use axum::{extract::State, TypedHeader, headers::{Authorization, authorization::Bearer}, response::IntoResponse, middleware::Next, http::Request};
use redis::{Client, Commands};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

lazy_static! {
    pub static ref KEY_ENCODING: EncodingKey = EncodingKey::from_secret("rarara".as_bytes());
    pub static ref KEY_DECODING: DecodingKey = DecodingKey::from_secret("rarara".as_bytes());
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: i64,
    pub exp: u64,
    pub iat: u64,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn my_middleware<B>(
    State(state): State<Client>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    /*获取method path */
    let action = req.method().clone().to_string();
    let path = req.uri().path().to_string();

    /*验证token有效性*/
    let token_data = decode::<Claims>(auth.token(), &KEY_DECODING, &Validation::default()).unwrap();

    // check if the token is in the blacklist
    let mut con = state.get_connection().unwrap();
    con.get::<&str, ()>(auth.token()).unwrap();

    /*token为空提示登录*/
}
