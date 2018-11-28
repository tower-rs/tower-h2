extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate h2;
extern crate http;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tower_h2;
extern crate tower_service;

use bytes::Bytes;
use futures::*;
use http::Request;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_h2::{Body, Server, RecvBody};
use tower_service::{Service};

type Response = http::Response<RspBody>;

struct RspBody(Option<Bytes>);

impl RspBody {
    fn new(body: Bytes) -> Self {
        RspBody(Some(body))
    }

    fn empty() -> Self {
        RspBody(None)
    }
}


impl Body for RspBody {
    type Data = Bytes;

    fn is_end_stream(&self) -> bool {
        self.0.as_ref().map(|b| b.is_empty()).unwrap_or(false)
    }

    fn poll_data(&mut self) -> Poll<Option<Bytes>, h2::Error> {
        let data = self.0
            .take()
            .and_then(|b| if b.is_empty() { None } else { Some(b) });
        Ok(Async::Ready(data))
    }
}

const ROOT: &'static str = "/";

#[derive(Debug)]
struct Svc;
impl Service<Request<RecvBody>> for Svc {
    type Response = Response;
    type Error = h2::Error;
    type Future = future::FutureResult<Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: Request<RecvBody>) -> Self::Future {
        let mut rsp = http::Response::builder();
        rsp.version(http::Version::HTTP_2);

        let uri = req.uri();
        if uri.path() != ROOT {
            let body = RspBody::empty();
            let rsp = rsp.status(404).body(body).unwrap();
            return future::ok(rsp);
        }

        let body = RspBody::new("heyo!".into());
        let rsp = rsp.status(200).body(body).unwrap();
        future::ok(rsp)
    }
}

#[derive(Debug)]
struct NewSvc;
impl Service<()> for NewSvc {
    type Response = Svc;
    type Error = ::std::io::Error;
    type Future = future::FutureResult<Svc, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _target: ()) -> Self::Future {
        future::ok(Svc)
    }
}

fn main() {
    drop(env_logger::init());

    tokio::run(lazy(|| {
        let executor = DefaultExecutor::current();
        let h2 = Server::new(NewSvc, Default::default(), executor);

        let addr = "[::1]:8888".parse().unwrap();
        let bind = TcpListener::bind(&addr).expect("bind");

        bind.incoming()
            .fold(h2, |mut h2, sock| {
                if let Err(e) = sock.set_nodelay(true) {
                    return Err(e);
                }

                tokio::spawn({
                    h2.serve(sock)
                        .map_err(|e| error!("h2 error: {:?}", e))
                });

                Ok(h2)
            })
            .map_err(|e| error!("serve error: {:?}", e))
            .map(|_| {})
    }));
}
