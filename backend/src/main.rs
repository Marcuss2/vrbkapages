use axum::extract::{MatchedPath, Path};
use axum::http::{Request, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::Router;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{event, info, info_span, Level, Span};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let dist_dir = std::env::var("DIST_DIRECTORY").unwrap_or("../dist".into());

    let app = Router::new()
        .route("/health_check", get(health_check))
        .fallback_service(get_service(ServeDir::new(dist_dir.clone()).not_found_service(ServeFile::new(dist_dir + "/index.html"))))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    event!(Level::INFO, "Request {} {} received.", request.method(), request.uri());
                })
                .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                    event!(
                        Level::INFO,
                        "Response sent, took {}ms.",
                        latency.as_millis()
                    );
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        event!(Level::ERROR, "Error: {}", error);
                    },
                ),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("Server is listening on port 3000");

    axum::serve(listener, app).await
}
