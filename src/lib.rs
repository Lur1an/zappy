use http_body_util::{BodyExt, Empty, Full, StreamBody};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::{service_fn, Service};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::future::Future;
use tokio::net::TcpListener;

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Empty<Bytes>>, Infallible> {
    //match (req.method(), req.uri().path()) {
    //    (&Method::GET, "/") => {}
    //}
    let resp = Response::new(Empty::new());
    Ok(resp)
}

#[derive(Clone)]
pub struct App<F> {
    handler: F,
}

impl<F> App<F> {
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

type Req = Request<hyper::body::Incoming>;

impl<Fut, F> Service<Req> for App<F>
where
    F: Fn(Req) -> Fut,
    Fut: Future<Output = Result<Response<Full<Bytes>>, Infallible>>,
{
    type Response = Response<Full<Bytes>>;

    type Error = Infallible;

    type Future = Fut;

    fn call(&self, req: Req) -> Self::Future {
        tracing::info!("request: {:?}", req);
        (self.handler)(req)
    }
}

pub async fn launch() -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    let f = |req| async move {
        Ok::<Response<Full<Bytes>>, Infallible>(Response::new(Full::new(Bytes::new())))
    };
    let s = App::new(f);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let s = s.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, s).await {
                tracing::error!(?err);
            }
        });
    }
}
