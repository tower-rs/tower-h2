use self::support::*;
extern crate tokio_connect;

use h2_support::prelude::*;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::runtime::current_thread::Runtime;
use tokio_current_thread::TaskExecutor;

use tower_h2::client::Connect;
use tower_h2::Body;

use tower_service::{NewService, Service};
use futures::future::{self, FutureResult};
use std::{
    cell::RefCell,
    io,
};

mod support;

struct MockConn<T, E> {
    conn: RefCell<Option<Result<T, E>>>,
}

impl<T> MockConn<T, io::Error> {
    fn new(mock: T) -> Self {
        MockConn {
            conn: RefCell::new(Some(Ok(mock)))
        }
    }
}

impl<E> MockConn<h2_support::mock::Mock, E> {
    fn error(error: E) -> Self {
        MockConn {
            conn: RefCell::new(Some(Err(error)))
        }
    }
}

impl<T: AsyncRead + AsyncWrite, E> tokio_connect::Connect for MockConn<T, E> {
    type Connected = T;
    type Error = E;
    type Future = FutureResult<Self::Connected, Self::Error>;

    fn connect(&self) -> Self::Future {
        let result = self.conn
            .borrow_mut()
            .take()
            .expect("connected more than once!");
        future::result(result)
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
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
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
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
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

#[test]
fn respects_flow_control_eos_signal() {
    use futures::Poll;
    use std::cell::Cell;
    use std::rc::Rc;

    let _ = ::env_logger::try_init();

    struct Zeros {
        buf: Bytes,
        cnt: Rc<Cell<usize>>,
    }

    impl Zeros {
        fn new(cnt: Rc<Cell<usize>>) -> Zeros {
            let buf = vec![0; 16_384].into();

            Zeros {
                buf,
                cnt,
            }
        }
    }

    impl Body for Zeros {
        type Data = Bytes;

        fn is_end_stream(&self) -> bool {
            self.cnt.get() == 5
        }

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, support::h2::Error> {
            let cnt = self.cnt.get();

            if cnt == 5 {
                panic!("the library should not have called this");
            } else {
                self.cnt.set(cnt + 1);
                Ok(Some(self.buf.clone()).into())
            }
        }
    }

    let (io, srv) = mock::new();
    let frame = vec![0; 16_384];
    let cnt = Rc::new(Cell::new(0));
    let cnt2 = cnt.clone();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..16383]))
        .idle_ms(100)
        .and_then(move |v| {
            assert_eq!(4, cnt2.get());
            Ok(v)
        })
        .send_frame(
            frames::window_update(0, 1_000_000)
        )
        .send_frame(
            frames::window_update(1, 1_000_000)
        )
        .recv_frame(frames::data(1, &frame[..1]))
        .recv_frame(frames::data(1, &frame[..]).eos())
        .send_frame(
            frames::headers(1)
                .response(200)
                .eos()
        )
        .close();

    let conn = MockConn::new(io);
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(Zeros::new(cnt.clone()))
                .unwrap())
        })
       .map(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
        })
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new().unwrap()
        .spawn(srv)
        .block_on(done)
        .unwrap();
}


#[test]
fn respects_flow_control_no_eos_signal() {
    use futures::Poll;
    use std::cell::Cell;
    use std::rc::Rc;

    let _ = ::env_logger::try_init();

    struct Zeros {
        buf: Bytes,
        cnt: Rc<Cell<usize>>,
    }

    impl Zeros {
        fn new(cnt: Rc<Cell<usize>>) -> Zeros {
            let buf = vec![0; 16_384].into();

            Zeros {
                buf,
                cnt,
            }
        }
    }

    impl Body for Zeros {
        type Data = Bytes;

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, support::h2::Error> {
            let cnt = self.cnt.get();

            if cnt == 5 {
                Ok(None.into())
            } else {
                self.cnt.set(cnt + 1);
                Ok(Some(self.buf.clone()).into())
            }
        }
    }

    let (io, srv) = mock::new();
    let frame = vec![0; 16_384];
    let cnt = Rc::new(Cell::new(0));
    let cnt2 = cnt.clone();

    let srv = srv
        .assert_client_handshake()
        .unwrap()
        .recv_settings()
        .recv_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &frame[..16383]))
        .idle_ms(100)
        .and_then(move |v| {
            assert_eq!(4, cnt2.get());
            Ok(v)
        })
        .send_frame(
            frames::window_update(0, 1_000_000)
        )
        .send_frame(
            frames::window_update(1, 1_000_000)
        )
        .recv_frame(frames::data(1, &frame[..1]))
        .recv_frame(frames::data(1, &frame[..]))
        .recv_frame(frames::data(1, &b""[..]).eos())
        .send_frame(
            frames::headers(1)
                .response(200)
                .eos()
        )
        .close();


    let conn = MockConn::new(io);
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
        .map_err(|e| panic!("connect err: {:?}", e))
        .and_then(|mut h2| {
            h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(Zeros::new(cnt.clone()))
                .unwrap())
        })
       .map(|rsp| {
            assert_eq!(rsp.status(), http::StatusCode::OK);
        })
        .map_err(|e| panic!("error: {:?}", e));

    Runtime::new().unwrap()
        .spawn(srv)
        .block_on(done)
        .unwrap();
}

#[test]
fn connect_error() {
    let _ = ::env_logger::try_init();
    let conn = MockConn::error(io::Error::from(io::ErrorKind::Other));
    let h2 = Connect::new(conn, Default::default(), TaskExecutor::current());

    let done = h2.new_service()
        .map_err(|e| {
            assert_eq!(
                format!("{}", e),
                "Error attempting to establish underlying session layer: other os error".to_string()
            );
        })
        .map(|mut h2| {
            let _ = h2.call(http::Request::builder()
                .method("GET")
                .uri("https://example.com/")
                .body(())
                .unwrap());
            panic!("got a connection that should have errored!");
        });

    Runtime::new().unwrap()
        .block_on(done)
        .unwrap_err();
}
