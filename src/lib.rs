mod service;

use std::convert::Infallible;
use std::sync::atomic::AtomicU32;
use std::time::Instant;

use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use rocksdb::{
    DBAccess, DBWithThreadMode, MultiThreaded, Options, ReadOptions, SingleThreaded, DB,
};
use tokio::net::TcpListener;

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Empty<Bytes>>, Infallible> {
    //match (req.method(), req.uri().path()) {
    //    (&Method::GET, "/") => {}
    //}
    let resp = Response::new(Empty::new());
    Ok(resp)
}

pub async fn launch() -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!(?addr, "Listening...");
    loop {
        let (stream, addr) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| async move {
                        Ok::<Response<Empty<Bytes>>, Infallible>(Response::new(Empty::new()))
                    }),
                )
                .await
            {
                tracing::error!(?err);
            }
        });
    }
}
