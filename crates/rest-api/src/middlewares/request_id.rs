use axum::http::HeaderName;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UuidRequestIdMaker;

pub(super) const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

impl MakeRequestId for UuidRequestIdMaker {
    fn make_request_id<B>(
        &mut self,
        _: &axum::http::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        let request_id = Uuid::new_v4()
            .to_string()
            .parse()
            .expect("UUID v4 is a valid HeaderValue");
        Some(RequestId::new(request_id))
    }
}

pub fn set_request_id_layer() -> SetRequestIdLayer<UuidRequestIdMaker> {
    SetRequestIdLayer::new(REQUEST_ID_HEADER.clone(), UuidRequestIdMaker)
}

pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
    PropagateRequestIdLayer::new(REQUEST_ID_HEADER.clone())
}
