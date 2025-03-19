mod handlers;

use axum::Router;
use axum::routing::get;




#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the configuration server on http://0.0.0.0:8080");

    let app = Router::new()
        .route("/", get(handlers::home_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
