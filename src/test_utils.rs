
use axum::{
    body::{Body, BoxBody, HttpBody},
    http::{header::CONTENT_TYPE, request, Request, Response},
};

pub trait RequestBuilderExt {
    fn json(self, json: serde_json::Value) -> Request<Body>;
    fn empty_body(self) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
    fn json(self, json: serde_json::Value) -> Request<Body> {
        let body = Body::from(json.to_string());

        self.header("Content-Type", "application/json")
            .body(body)
            .expect("failed to buld request")
    }

    fn empty_body(self) -> Request<Body> {
        self.body(Body::empty()).expect("failed to build request")
    }
}

#[track_caller]
pub async fn response_json(resp: &mut Response<BoxBody>) -> serde_json::Value {
    assert_eq!(
        resp.headers()
            .get(CONTENT_TYPE)
            .expect("expected Content-Type"),
        "application/json"
    );

    let body = resp.body_mut();
    let mut bytes = Vec::new();

    while let Some(res) = body.data().await {
        let chunk = res.expect("error reading response body");
        bytes.extend_from_slice(&chunk[..]);
    }

    serde_json::from_slice(&bytes).expect("Failed to read response body as json")
}
