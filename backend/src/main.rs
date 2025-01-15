use std::time::Duration;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{event, info, info_span, Level, Span};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/health_check", get(health_check))
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
                    event!(Level::INFO, "Request received.");
                })
                .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                    event!(Level::INFO, "Response sent, took {}ms.", latency.as_millis());
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
