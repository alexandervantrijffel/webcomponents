use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{Level, Span};

pub fn add_middleware_layers<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let router = router.layer(
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                // Cross-Origin Request Warning: The Same Origin Policy will disallow reading the remote resource at http://localhost:8081/api/std/new soon. (Reason: When the `Access-Control-Allow-Headers` is `*`, the `Authorization` header is not covered. To include the `Authorization` header, it must be explicitly listed in CORS header `Access-Control-Allow-Headers`).
                .allow_headers(Any),
        ),
    );
    let router = add_tracing_middleware(router);
    router.layer(
        ServiceBuilder::new().layer(NormalizePathLayer::trim_trailing_slash()), // .map_response_body(axum::body::boxed)
    )
}

fn add_tracing_middleware<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Add high level tracing/logging to all requests
    router
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(
            TraceLayer::new_for_http()
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(LatencyUnit::Micros),
                )
                // .on_request(|request: &Request<_>, _span: &Span| {
                //     debug!("req: {}", request.uri());
                // })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("Server error: {:?}", error)
                    },
                ),
        )
}
