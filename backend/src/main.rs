use axum::{routing::get, Router};
use tower_http::{
  cors::{Any, CorsLayer},
  trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod errors;

fn init_tracing() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();
}

#[tokio::main]
async fn main() {
  init_tracing();
  tracing::info!("Backend started");

  let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any)
    .allow_headers(Any);

  let app = Router::new()
    .route("/statusz", get(statusz))
    .layer(cors)
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

  axum::Server::bind(&format!("0.0.0.0:{}", 8080).parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn statusz() -> errors::Result<String> {
  Ok("OK".to_owned())
}
