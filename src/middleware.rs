use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::Request,
    middleware::Next,
    response::IntoResponse,
    TypedHeader,
};
use chrono::Local;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use redis::{Client, Commands};
use serde::{Deserialize, Serialize};

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

    let mut con = state.get_connection().unwrap();

    /*验证token有效性*/
    let token_data = decode::<Claims>(auth.token(), &KEY_DECODING, &Validation::default()).unwrap();
    if token_data.claims.exp < Local::now().timestamp() as u64 {
        match con.sadd::<&str, &str, ()>("expired_xuanqi_jwt", auth.token()) {
            Ok(_) => (),
            Err(_) => (),
        };
        return "Please login again".into_response()
    }

    // check if the token is in the blacklist
    // con.get::<&str, ()>(auth.token()).unwrap();

    /*token为空提示登录*/
    // do something with `req`...
    let res = next.run(req).await;
    // do something with `res`...
    res
}
