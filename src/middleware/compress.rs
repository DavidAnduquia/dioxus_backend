use axum::{body::Bytes, middleware::Next, response::Response};
use flate2::{Compression, bufread::GzEncoder};
use std::io::Read;

pub async fn compress_response<B>(body: B, next: Next<B>) -> Response {
    let mut response = next.run(body).await;
    
    if let Some(body) = response.body_mut().as_mut() {
        let mut encoder = GzEncoder::new(&body[..], Compression::fast());
        let mut compressed = Vec::new();
        encoder.read_to_end(&mut compressed).unwrap();
        *body = compressed.into();
        response.headers_mut().insert("Content-Encoding", "gzip".parse().unwrap());
    }
    
    response
}
