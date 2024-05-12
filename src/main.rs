use std::sync::Arc;
use axum::{
    extract::{Path, State}, 
    routing::{delete, get}, 
    Json, 
    Router
};

use crate::handler::WebHandler;
use crate::model::Greeting;

mod handler;
mod model;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(WebHandler::default());

    let app = Router::new()
    .route("/hello/:visitor", get(greet_visitor))
    .route("/bye", delete(say_goodbye))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet_visitor(
    State(handler): State<Arc<WebHandler>>,
    Path(visitor): Path<String>,
) -> Json<Greeting> {
    Json(handler.greet(visitor))
}

async fn say_goodbye(
    State(handler): State<Arc<WebHandler>>) -> String {
        handler.say_goodbye()
}

