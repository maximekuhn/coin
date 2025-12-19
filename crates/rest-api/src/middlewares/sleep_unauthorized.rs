use std::time::Duration;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use rand::Rng;

pub async fn sleep_on_401(request: Request, next: Next) -> impl IntoResponse {
    let res = next.run(request).await;
    if res.status() == StatusCode::UNAUTHORIZED {
        let delay = Duration::from_millis(rand::rng().random_range(500..=900));
        tokio::time::sleep(delay).await;
    }
    res
}
