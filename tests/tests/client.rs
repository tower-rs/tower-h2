use self::support::*;
extern crate tokio_connect;

use h2_support::{mock::Mock, prelude::*};
use tokio::executor::current_thread::*;

use tower_h2::client::Connect;

use tower_service::{NewService, Service};
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

impl tokio_connect::Connect for MockConn {
    type Connected = Mock;
    type Error = ::std::io::Error;
    type Future = FutureResult<Mock, ::std::io::Error>;

    fn connect(&self) -> Self::Future {
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
                .eos(),
        )
        .send_frame(frames::headers(1).response(200).eos())
        .close();

    let conn = MockConn::new(io);
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(())
                .unwrap())
        })
        .map(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
        })
        .map_err(|e| panic!("error: {:?}", e));

    CurrentThread::new()
        .spawn(srv)
        .block_on(done)
        .unwrap();
}
