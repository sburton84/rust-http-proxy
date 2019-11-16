use hyper::{header, Body, Request};
use crate::error::NoHostError;

pub fn remove_proxy_headers(req: &mut Request<Body>) {
    // Remove headers that shouldn't be forwarded to upstream
    req.headers_mut().remove(header::ACCEPT_ENCODING);
    req.headers_mut().remove(header::CONNECTION);
    req.headers_mut().remove("proxy-connection");
    req.headers_mut().remove(header::PROXY_AUTHENTICATE);
    req.headers_mut().remove(header::PROXY_AUTHORIZATION);
}

pub fn add_proxy_headers(req: &mut Request<Body>) {
    // TODO: Add X-Forwarded-For header
    // TODO: Add Via header
}

pub fn get_host(req: &Request<Body>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Attempt to get Host header
    let host = req.headers().get(header::HOST);

    // Get the Authority in the request start line
    let uri = req.uri().authority_part();

    // TODO: Appropriate logic based on https://tools.ietf.org/html/rfc7230#section-5.4
    match (host, uri) {
        (Some(host), None) => {
            Ok(host.to_str()?.to_string())
        },
        (None, Some(authority)) => {
            Ok(authority.as_str().to_string())
        },
        (Some(_), Some(authority)) => {
            Ok(authority.as_str().to_string())
        },
        (None, None) => Err(Box::new(NoHostError)),
    }
}
