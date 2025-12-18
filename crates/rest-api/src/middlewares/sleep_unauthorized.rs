use std::time::Duration;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn sleep_on_401(request: Request, next: Next) -> impl IntoResponse {
    let res = next.run(request).await;
    if res.status() == StatusCode::UNAUTHORIZED {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    res
}
