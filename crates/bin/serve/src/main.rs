use anyhow::{Context, Result};
use askama::Template;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use chrono::NaiveDate;
use web_runner::axum_server::add_middleware_layers;
use web_runner::layout::LayoutTemplate;
use web_runner::start_listenfd;
use webcomponents::calendar::{DayConfig, DayMarker};
use webcomponents::{get_base_css, CalendarTemplate};

include!("generated_at_build.rs");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState::new().await?;

    // let assets_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("assets");
    // println!("assets folder: {}", assets_dir.display());

    // let serve_dir = ServeDir::new(assets_dir.clone())
    //     .not_found_service(ServeFile::new(assets_dir.join("404.html")));

    let router = add_middleware_layers(
        Router::new()
            .route("/", get(view_layout))
            .route("/assets/css/base.css", get(serve_base_css))
            // .nest_service("/assets", serve_dir)
            .with_state(app_state),
    );

    axum::serve(start_listenfd("0.0.0.0:8181").await?, router)
        .await
        .context("Failed to run axum server")
}

async fn serve_base_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, get_base_css().to_vec())
}

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    async fn new() -> Result<Self> {
        Ok(Self {})
    }
}

async fn view_layout() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            LayoutTemplate {
                build_id: crate::BUILD_ID,
                #[allow(deprecated)]
                component: CalendarTemplate::try_new(
                    10,
                    2024,
                    vec![
                        DayConfig {
                            date: NaiveDate::from_ymd(2024, 10, 7),
                            marker: Some(DayMarker::Green),
                        },
                        DayConfig {
                            date: NaiveDate::from_ymd(2024, 10, 8),
                            marker: None,
                        },
                        DayConfig {
                            date: NaiveDate::from_ymd(2024, 10, 9),
                            marker: Some(DayMarker::Orange),
                        },
                    ],
                )
                .unwrap(),
            }
            .render()
            .unwrap(),
        ),
    )
}
