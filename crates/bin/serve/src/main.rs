use anyhow::{Context, Result};
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use web_runner::axum_server::add_middleware_layers;
use web_runner::start_listenfd;
use webcomponents::CalendarTemplate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState::new().await?;

    let router = add_middleware_layers(
        Router::new()
            .route("/", get(view_calendar))
            // .route("/calendar", get(view_calendar::view_calendar))
            .with_state(app_state),
    );

    axum::serve(start_listenfd("0.0.0.0:8181").await?, router)
        .await
        .context("Failed to run axum server")
}

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    async fn new() -> Result<Self> {
        Ok(Self {})
    }
}

pub async fn view_calendar() -> impl IntoResponse {
    (StatusCode::OK, Html(CalendarTemplate {}.render().unwrap()))
}
