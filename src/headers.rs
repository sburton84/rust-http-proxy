use {
    hyper::{Body, Request, header},
};

pub fn remove_proxy_headers(req: &mut Request<Body>) {
    // Remove headers that shouldn't be forwarded to upstream
    req.headers_mut().remove(header::ACCEPT_ENCODING);
    req.headers_mut().remove(header::CONNECTION);
    req.headers_mut().remove("proxy-connection");
    req.headers_mut().remove(header::PROXY_AUTHENTICATE);
    req.headers_mut().remove(header::PROXY_AUTHORIZATION);
}
