use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;

type Req = Request<Incoming>;

pub trait Persistence {}

pub struct App<P> {
    persistence: P,
}
