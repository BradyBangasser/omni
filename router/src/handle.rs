use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming};

pub async fn handle_request(
    _req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from(
        "Hello from the Share-Nothing Monolith!",
    ))))
}
