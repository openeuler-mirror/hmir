use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use hmir_ws_client_mgr::ws_client_mgr::register_client;
use serde::{Deserialize, Serialize};
use svr::svr_cmd::cmd_all_slice;
use std::net::SocketAddr;

mod svr;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    register_client("127.0.0.1", 8080);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/login",get(login))
        .route("/service",get(service))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5899").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn service() -> String {
    let (_a,b) = cmd_all_slice("127.0.0.1");
    b
}

async fn login(query: Query<LoginInfo>) -> String {
    let info = query.0;
    format!("query1: {info:?}")
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Deserialize,Debug)]
struct LoginInfo {
    username: String,
    password: String,
}


// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}