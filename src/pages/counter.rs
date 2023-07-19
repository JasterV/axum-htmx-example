use crate::pages::HtmlTemplate;
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

pub struct CounterState(AtomicUsize);

impl CounterState {
    pub fn new() -> Self {
        CounterState(AtomicUsize::new(0))
    }
}

#[derive(Template)]
#[template(path = "counter.html")]
pub struct CounterTemplate {
    pub count: usize,
}

pub async fn view(State(state): State<Arc<CounterState>>) -> impl IntoResponse {
    let template = CounterTemplate {
        count: state.0.load(std::sync::atomic::Ordering::Relaxed),
    };
    HtmlTemplate::new(template)
}

pub async fn increment(State(state): State<Arc<CounterState>>) -> impl IntoResponse {
    let increment = 1;
    let previous = state
        .0
        .fetch_add(increment, std::sync::atomic::Ordering::Relaxed);

    tokio::time::sleep(Duration::from_secs(1)).await;

    Html(format!("{}", previous + increment))
}
