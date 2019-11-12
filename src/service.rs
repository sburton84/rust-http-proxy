use {
    crate::connection::State,
    crate::headers::remove_proxy_headers,
    http::uri::Scheme,
    hyper::{client::HttpConnector, Client, header, Body, Method, Request, Response, StatusCode},
    hyper_tls::HttpsConnector,
    log::trace,
    std::future::Future,
    std::sync::{Arc, Mutex},
    std::task::Poll,
    std::pin::Pin,
    tower_service::Service,
    futures_util::FutureExt,
};

/// Uses Hyper to handle the initial HTTP request
///
/// If it is not a CONNECT request, it will handle forwarding the request
/// to the upstream server and returning the response.
///
/// If it is a CONNECT request it will delegate TLS tunneling by setting
/// the state variable `tunnel` to `true` and returning HTTP status 200.
#[derive(Clone)]
pub struct ProxyService {
    state: Arc<Mutex<State>>,
    connector: HttpsConnector<HttpConnector>,
}

impl ProxyService {
    pub fn new(state: Arc<Mutex<State>>, connector: HttpsConnector<HttpConnector>) -> Self {
        ProxyService {
            state: state,
            connector: connector,
        }
    }

    async fn handle_connect(self, mut req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap())
    }

    async fn handle_proxy(self, mut req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        // Remove headers that shouldn't be forwarded to upstream
        remove_proxy_headers(&mut req);

        // Make request to upstream server and return the response to the client
        let client = Client::builder().build::<_, Body>(self.connector);
        let response = client.request(req).await;

        response.or_else(|e| {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap())
        })
    }
}

impl Service<Request<Body>> for ProxyService {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;
    type Response = Response<Body>;

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        trace!("Received request:\n{:#?}", req);

        if req.method() == Method::CONNECT {
            self.clone().handle_connect(req).boxed()
        } else {
            self.clone().handle_proxy(req).boxed()
        }
    }

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}