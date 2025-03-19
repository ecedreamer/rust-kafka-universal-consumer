use axum::response::{Html, IntoResponse, Response};


pub async fn home_handler() -> impl IntoResponse {
    tracing::info!("This is a home page");

    let resp_str = "<h3>This is home page</h3>";
    Html::from(resp_str)
}