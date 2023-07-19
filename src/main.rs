mod configuration;
mod error;
mod pages;

use axum::{
    routing::{get, post},
    Router,
};
use configuration::Configuration;
use pages::counter::CounterState;
use std::sync::Arc;

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