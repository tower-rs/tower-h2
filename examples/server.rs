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
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tower_h2::{Body, Server, RecvBody};
use tower_service::{NewService, Service};

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
impl Service for Svc {
    type Request = Request<RecvBody>;
    type Response = Response;
    type Error = h2::Error;
    type Future = future::FutureResult<Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
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
impl NewService for NewSvc {
    type Request = Request<RecvBody>;
    type Response = Response;
    type Error = h2::Error;
    type InitError = ::std::io::Error;
    type Service = Svc;
    type Future = future::FutureResult<Svc, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(Svc)
    }
}

fn main() {
    drop(env_logger::init());

    let mut rt = Runtime::new().unwrap();
    let reactor = rt.executor();

    let h2 = Server::new(NewSvc, Default::default(), reactor.clone());

    let addr = "[::1]:8888".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind.incoming()
        .fold((h2, reactor), |(h2, reactor), sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            reactor.spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok((h2, reactor))
        })
        .map_err(|e| error!("serve error: {:?}", e))
        .map(|_| {})
        ;

    rt.spawn(serve);
    rt.shutdown_on_idle()
        .wait().unwrap();
}
