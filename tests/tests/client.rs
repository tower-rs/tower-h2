use self::support::*;
extern crate tokio_connect;

use h2_support::{mock::Mock, prelude::*};
use tokio::runtime::current_thread::Runtime;
use tokio_current_thread::TaskExecutor;
use tower_h2::client::Connect;
use tower_h2::NoBody;

use tower_service::Service;
use tower::MakeService;
use futures::Poll;
use futures::future::{self, FutureResult};
use std::cell::RefCell;

mod support;

struct MockConn {
    conn: RefCell<Option<Mock>>,
}

impl MockConn {
    fn new(mock: Mock) -> Self {
        MockConn {
            conn: RefCell::new(Some(mock))
        }
    }
}

impl Service<()> for MockConn {
    type Response = Mock;
    type Error = ::std::io::Error;
    type Future = FutureResult<Mock, ::std::io::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        future::ok(self.conn.borrow_mut().take().expect("connected more than once!"))
    }
}

#[test]
fn hello() {
    let _ = ::env_logger::try_init();

    let (io, srv) = mock::new();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos()
        )
        .send_frame(frames::headers(1).response(200).eos())
        .close();

    let conn = MockConn::new(io);
    let mut h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.make_service(())
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(NoBody)
                .unwrap())
        })
        .map(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
        })
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new()
        .unwrap()
        .block_on(done.join(srv))
        .unwrap();
}

#[test]
fn hello_req_body() {
    let _ = ::env_logger::try_init();

    let (io, srv) = mock::new();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .recv_frame(frames::data(1, "hello world").eos())
        .send_frame(frames::headers(1).response(200).eos())
        .close();

    let conn = MockConn::new(io);
    let mut h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.make_service(())
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(SendBody::new("hello world"))
                .unwrap())
        })
        .map(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
        })
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new()
        .unwrap()
        .block_on(done.join(srv))
        .unwrap();
}

#[test]
fn hello_rsp_body() {
    let _ = ::env_logger::try_init();

    let (io, srv) = mock::new();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos()
        )
        .send_frame(frames::headers(1).response(200))
        .send_frame(frames::data(1, "hello world").eos())
        .close();

    let conn = MockConn::new(io);
    let mut h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.make_service(())
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(NoBody)
                .unwrap())
        })
        .and_then(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
            let (_, body) = rsp.into_parts();
            read_recv_body(body).from_err()
        })
        .map(|body| assert_eq!(body, Some("hello world".into())))
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new()
        .unwrap()
        .block_on(done.join(srv))
        .unwrap();
}

#[test]
fn hello_bodies() {
    let _ = ::env_logger::try_init();

    let (io, srv) = mock::new();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .recv_frame(frames::data(1, "hello world").eos())
        .send_frame(frames::headers(1).response(200))
        .send_frame(frames::data(1, "hello"))
        .send_frame(frames::data(1, " back!").eos())
        .close();

    let conn = MockConn::new(io);
    let mut h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.make_service(())
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(SendBody::new("hello world"))
                .unwrap())
        })
        .and_then(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
            let (_, body) = rsp.into_parts();
            read_recv_body(body).from_err()
        })
        .map(|body| assert_eq!(body, Some("hello back!".into())))
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new()
        .unwrap()
        .block_on(done.join(srv))
        .unwrap();
}
