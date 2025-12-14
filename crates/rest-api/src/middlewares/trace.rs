use axum::{body::Body, http::Request};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::Span;

pub fn trace_layer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, impl Fn(&Request<Body>) -> Span + Clone> {
    TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
        let req_id = req
            .headers()
            .get(super::request_id::REQUEST_ID_HEADER.as_str())
            .and_then(|v| v.to_str().ok())
            .unwrap_or("not-defined");
        tracing::info_span!(
            "request", 
            x_request_id = req_id, 
            method = %req.method(),
            uri = %req.uri())
    })
}
