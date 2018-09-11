extern crate bytes;
#[macro_use]
extern crate futures;
extern crate h2;
extern crate h2_support;
extern crate http;
extern crate tokio;
extern crate tower_h2;
extern crate tower_service;
extern crate tower_util;

use bytes::{Bytes, Buf};
use h2_support::prelude::*;
use tokio::executor::current_thread::*;
use tower_h2::{Body, RecvBody};
use tower_h2::server::Server;
use futures::{Poll, Async};

pub struct RspBody(Option<Bytes>);

impl RspBody {
    pub fn new(body: Bytes) -> Self {
        RspBody(Some(body))
    }

    pub fn empty() -> Self {
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


pub fn read_recv_body(body: RecvBody) -> ReadRecvBody {
    ReadRecvBody {
        body,
        bytes: None,
    }
}
pub struct ReadRecvBody {
    body: RecvBody,
    bytes: Option<Box<Buf>>,
}

impl Future for ReadRecvBody {
    type Item = Option<Bytes>;
    type Error = h2::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            self.bytes = match try_ready!(self.body.poll_data()) {
                None => return Ok(Async::Ready(self.bytes.take().map(Buf::collect))),
                Some(b) => if self.bytes.as_ref().is_none() {
                    Some(Box::new(b))
                } else {
                    Some(Box::new(self.bytes.take().unwrap().chain(b)))
                },
            }
        }
    }
}

mod extract {
    use futures::{Async, Poll, IntoFuture};
    use futures::future::{self, FutureResult};
    use tower_service::{Service, NewService};

    use std::marker::PhantomData;
    use std::sync::Arc;

    pub struct SyncServiceFn<T, R> {
        f: Arc<T>,
        // don't impose Sync on R
        _ty: PhantomData<fn() -> R>,
    }

    impl<T, R, S> SyncServiceFn<T, R>
    where T: Fn(R) -> S,
          S: IntoFuture,
    {
        pub fn new(f: T) -> Self {
            SyncServiceFn {
                f: Arc::new(f),
                _ty: PhantomData,
            }
        }
    }

    impl<T, R, S> Service for SyncServiceFn<T, R>
    where T: Fn(R) -> S,
          S: IntoFuture,
    {
        type Request = R;
        type Response = S::Item;
        type Error = S::Error;
        type Future = S::Future;

        fn poll_ready(&mut self) -> Poll<(), Self::Error> {
            Ok(Async::Ready(()))
        }

        fn call(&mut self, request: Self::Request) -> Self::Future {
            (self.f)(request).into_future()
        }
    }

    impl<T, R, S> NewService for SyncServiceFn<T, R>
    where T: Fn(R) -> S,
          S: IntoFuture,
    {
        type Request = R;
        type Response = S::Item;
        type Error = S::Error;
        type Service = Self;
        type InitError = ();
        type Future = FutureResult<Self, ()>;

        fn new_service(&self) -> Self::Future {
            future::ok(self.clone())
        }
    }

    impl<T, R, S> Clone for SyncServiceFn<T, R>
    where T: Fn(R) -> S,
          S: IntoFuture,
    {
        fn clone(&self) -> Self {
            SyncServiceFn {
                f: self.f.clone(),
                _ty: PhantomData,
            }
        }
    }
}
use extract::*;

#[test]
fn hello() {
    let _ = ::env_logger::try_init();

    let (io, client) = mock::new();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos(),
        )
        .recv_frame(frames::headers(1).response(200).eos())
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(|_request| {
            let response = http::Response::builder()
                .status(200)
                .body(())
                .unwrap();

            Ok::<_, ()>(response.into())
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
        .unwrap();
}

#[test]
fn hello_bodies() {
    let _ = ::env_logger::try_init();

    let (io, client) = mock::new();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .send_frame(
            frames::data(1, "hello world").eos()
        )
        .recv_frame(frames::headers(1).response(200))

        .recv_frame(frames::data(1, "hello back"))
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(|request: http::Request<tower_h2::RecvBody>| {
            let (_, body) = request.into_parts();
            read_recv_body(body)
                .and_then(|body| {
                    assert_eq!(body, Some("hello world".into()));

                    let response = http::Response::builder()
                        .status(200)
                        .body(RspBody::new("hello back".into()))
                        .unwrap();
                    Ok(response)
                })
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
        .unwrap();
}


#[test]
fn hello_rsp_body() {
    let _ = ::env_logger::try_init();

    let (io, client) = mock::new();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos()
        )
        .recv_frame(frames::headers(1).response(200))
        .recv_frame(frames::data(1, "hello back"))
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(|_req| {
            let response = http::Response::builder()
                .status(200)
                .body(RspBody::new("hello back".into()))
                .unwrap();

            Ok::<_, ()>(response.into())
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
        .unwrap();
}

#[test]
fn hello_req_body() {
    let _ = ::env_logger::try_init();

    let (io, client) = mock::new();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
        )
        .send_frame(frames::data(1, "hello "))
        .send_frame(frames::data(1, "world").eos())
        .recv_frame(frames::headers(1).response(200).eos())
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(|request: http::Request<tower_h2::RecvBody>| {
            let (_, body) = request.into_parts();
            read_recv_body(body)
                .and_then(|body| {
                    assert_eq!(body, Some("hello world".into()));

                    let response = http::Response::builder()
                        .status(200)
                        .body(())
                        .unwrap();
                    Ok(response)
                })
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
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

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            let cnt = self.cnt.get();

            if cnt == 5 {
                panic!("the library should not have called this");
            } else {
                self.cnt.set(cnt + 1);
                Ok(Some(self.buf.clone()).into())
            }
        }
    }

    let (io, client) = mock::new();
    let frame = vec![0; 16_384];
    let cnt = Rc::new(Cell::new(0));
    let cnt2 = cnt.clone();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos(),
        )
        .recv_frame(frames::headers(1).response(200))
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
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(move |_request| {
            let response = http::Response::builder()
                .status(200)
                .body(Zeros::new(cnt.clone()))
                .unwrap();

            Ok::<_, ()>(response.into())
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
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

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            let cnt = self.cnt.get();

            if cnt == 5 {
                Ok(None.into())
            } else {
                self.cnt.set(cnt + 1);
                Ok(Some(self.buf.clone()).into())
            }
        }
    }

    let (io, client) = mock::new();
    let frame = vec![0; 16_384];
    let cnt = Rc::new(Cell::new(0));
    let cnt2 = cnt.clone();

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos(),
        )
        .recv_frame(frames::headers(1).response(200))
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
        .close();

    let h2 = Server::new(
        SyncServiceFn::new(move |_request| {
            let response = http::Response::builder()
                .status(200)
                .body(Zeros::new(cnt.clone()))
                .unwrap();

            Ok::<_, ()>(response.into())
        }),
        Default::default(), TaskExecutor::current());

    CurrentThread::new()
        .spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)))
        .block_on(client)
        .unwrap();
}

#[test]
fn flushing_body_cancels_if_reset() {
    use futures::{Async, Poll};
    use std::rc::Rc;
    use std::cell::Cell;

    let _ = ::env_logger::try_init();

    struct Zeros {
        buf: Bytes,
        cnt: usize,
        dropped: Rc<Cell<usize>>,
    }

    impl Zeros {
        fn new(dropped: Rc<Cell<usize>>) -> Zeros {
            let buf = vec![0; 16_384].into();

            Zeros {
                buf,
                cnt: 0,
                dropped,
            }
        }
    }

    impl Drop for Zeros {
        fn drop(&mut self) {
            let n = self.dropped.get();
            self.dropped.set(n + 1);
        }
    }

    impl Body for Zeros {
        type Data = Bytes;

        fn is_end_stream(&self) -> bool {
            false
        }

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            if self.cnt == 1 {
                Ok(Async::NotReady)
            } else {
                self.cnt += 1;
                Ok(Some(self.buf.clone()).into())
            }
        }
    }

    let (io, client) = mock::new();
    let frame = vec![0; 16_384];

    let client = client
        .assert_server_handshake()
        .unwrap()
        .recv_settings()
        .send_frame(
            frames::headers(1)
                .request("GET", "https://example.com/")
                .eos(),
        )
        .recv_frame(frames::headers(1).response(200))
        .recv_frame(frames::data(1, &frame[..]))
        .idle_ms(10)
        .send_frame(frames::reset(1).cancel())
        .idle_ms(10)
        .close();

    let dropped = Rc::new(Cell::new(0));
    let dropped2 = dropped.clone();

    let h2 = Server::new(
        SyncServiceFn::new(move |_request| {
            let response = http::Response::builder()
                .status(200)
                .body(Zeros::new(dropped2.clone()))
                .unwrap();

            Ok::<_, ()>(response.into())
        }),
        Default::default(), TaskExecutor::current());

    // hold on to the runtime so that after block_on, it isn't dropped
    // immediately, which defeats our test.
    let mut rt = CurrentThread::new();
    rt.spawn(h2.serve(io).map_err(|e| panic!("err={:?}", e)));
    rt.block_on(client).unwrap();

    // The flush future should have finished, since it was reset. If so,
    // it will have dropped once.
    assert_eq!(dropped.get(), 1);
}
