// File Imports
use auth::{with_auth, Role};
use error::Error::*;
// Crate Imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/* <Infallible> is an empty error type in Rust that represents a filter that can never fail. */
use std::convert::Infallible; 
use std::sync::Arc;
use warp::{reject, reply, Filter, Rejection, Reply};

/* <reject> provides functions to generate rejections which represent failed HTTP requests */
/* <reply> provides functions to generate HTTP responses */
/* <Filters> modifies and filters requests before they reach a route handler. */
/* <Rejection> type represents a failed filter or route */
/* <Reply> type represents a successful HTTP reply */

mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
// Since, Login is checked using Rust, So, the JSOn formant must be Deserialized 
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
// Our backend is sending Response. So, it must be serialized.
pub struct LoginResponse {
    pub token: String,
}

#[tokio::main]
async fn main() {
    /* <<init_user()>> : Create Default Users */
    /* <<Arc>>: To have multiple ownership of some data */
    let users = Arc::new(init_user()); // Done to have a testing data...

    let login_route = warp::path!("login") // > matches HTTP requests on the /login path.
        .and(warp::post()) // > filters to only match POST requests.
        .and(with_users(users.clone()))
        .and(warp::body::json()) // > makes sure the request body is JSON which can be deserialized.
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

////////////////
// with_users //
////////////////
fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone()) // Returns a cloned users per request
    /* <warp.any()> Matches any incoming request */
}

///////////////////        
// login_handler //
///////////////////
pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
    .iter()
    .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = auth::create_jwt_token(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?; // <map_err> allows to transform the error type from one kind to another
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

//////////////////        
// user_handler //
//////////////////
pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

///////////////////        
// admin_handler //
///////////////////
pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

///////////////        
// init_user //
///////////////
fn init_user() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@user.com"),
            pw: String::from("user1"),
            role: String::from("User"),
        },
    );

    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@admin.com"),
            pw: String::from("admin1"),
            role: String::from("Admin"),
        },
    );
    return map;
}
