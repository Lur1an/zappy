use http_body_util::{BodyExt, Empty, Full, StreamBody};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::{service_fn, Service};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use sqlx::PgPool;
use std::convert::Infallible;
use std::future::Future;
use tokio::net::TcpListener;

async fn hello(
    pgpool: PgPool,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    todo!()
}

pub struct App<F> {
    handler: F,
    pool: PgPool,
}

struct Inner<F> {
    handler: F,
    pool: PgPool,
}

impl<F> App<F> {
    pub fn new(handler: F, pool: PgPool) -> Self {
        Self { handler, pool }
    }
}

type Req = Request<hyper::body::Incoming>;

impl<Fut, F> Service<Req> for App<F>
where
    F: Fn(PgPool, Req) -> Fut,
    Fut: Future<Output = Result<Response<Full<Bytes>>, Infallible>>,
{
    type Response = Response<Full<Bytes>>;

    type Error = Infallible;

    type Future = F::Output;

    fn call(&self, req: Req) -> Self::Future {
        let (x, y) = req.into_parts();
        todo!()
        //(self.handler)(self.pool.clone(), req)
    }
}

pub async fn launch(pool: PgPool) -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    let s: &'static App<_> = Box::leak(Box::new(App::new(hello, pool)));
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, s).await {
                tracing::error!(?err);
            }
        });
    }
}
