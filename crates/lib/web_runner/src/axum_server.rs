use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn add_middleware_layers<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                // Cross-Origin Request Warning: The Same Origin Policy will disallow reading the remote resource at http://localhost:8081/api/std/new soon. (Reason: When the `Access-Control-Allow-Headers` is `*`, the `Authorization` header is not covered. To include the `Authorization` header, it must be explicitly listed in CORS header `Access-Control-Allow-Headers`).
                .allow_headers(Any),
        ),
    )
}
