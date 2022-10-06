use crate::model::database::{add_user, verify_user, DB_NAME, USERS_COLL};
use crate::web_service::authentication::redisClient;
use axum::{
    extract::{Extension, Path},
    http::{Method, StatusCode},
    response::IntoResponse,
    Form,
};
use axum_database_sessions::{AxumRedisPool, AxumSession};
use axum_sessions_auth::AuthSession;
use mongodb::Client;

use super::{UserForm, UserInSession};

pub async fn login1(
    Form(input): Form<UserForm>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    let user = crate::entities::User::new(input.username, input.auth);

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
// We can get the Method to compare with what Methods we allow. Useful if this supports multiple methods.
// When called auth is loaded in the background for you.
// TODO add remaining time support
pub async fn login(
    // session: AxumSession<AxumRedisPool>,
    auth: AuthSession<UserInSession, i64, AxumRedisPool, redisClient>,
    Form(input): Form<UserForm>,
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

pub async fn logout(
    session: AxumSession<AxumRedisPool>,
    auth: AuthSession<UserInSession, i64, AxumRedisPool, redisClient>,
) -> impl IntoResponse {
    if auth.is_authenticated() {
        // Set the user ID of the User to the Session so it can be Auto Loaded the next load or redirect
        auth.logout_user().await;
        (StatusCode::OK, "ok").into_response()
    } else {
        (StatusCode::OK, "no-need").into_response()
    }
}
