mod config;
mod pages;

use crate::config::Configuration;
use axum::{
    routing::{get, post},
    Router,
};
use pages::counter::CounterState;
use std::sync::Arc;
use tower_http::services::ServeDir;

struct AppState {
    counter: Arc<CounterState>,
}

fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/counter",
            get(pages::counter::view).with_state(state.counter.clone()),
        )
        .route(
            "/counter/increment",
            post(pages::counter::increment).with_state(state.counter),
        )
        .nest_service("/assets", ServeDir::new("assets"))
}

#[tokio::main]
async fn main() {
    let config = Configuration::load().expect("Error loading configuration");
    let address = config.address();
    let state = AppState {
        counter: Arc::new(CounterState::new()),
    };

    axum::Server::bind(&address)
        .serve(router(state).into_make_service())
        .await
        .unwrap();
}
