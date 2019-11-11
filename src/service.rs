use {
    crate::connection::State,
    hyper::{Body, Request, Response, StatusCode},
    log::trace,
    std::future::Future,
    std::sync::{Arc, Mutex},
    std::task::Poll,
    tower_service::Service,
};

pub struct ProxyService {
    state: Arc<Mutex<State>>,
}

impl ProxyService {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        ProxyService { state }
    }
}

impl Service<Request<Body>> for ProxyService {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Unpin>;
    type Response = Response<Body>;

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        trace!("Received request:\n{:#?}", req);

        Box::new(futures_util::future::ok(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap(),
        ))
    }

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
