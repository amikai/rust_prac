use axum::{routing, Router};
use tower_http::trace::TraceLayer;

use anyhow::Result;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let r = Router::new().route("/hello", routing::get(hello)).layer(
        ServiceBuilder::new()
            // Add high level tracing/logging to all requests
            .layer(TraceLayer::new_for_http()),
    );
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(r.into_make_service())
        .await?;
    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}
