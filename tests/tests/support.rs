pub extern crate bytes;
pub extern crate futures;
pub extern crate h2;
pub extern crate h2_support;
pub extern crate http;
pub extern crate tokio;
pub extern crate tokio_current_thread;
pub extern crate tower_h2;
pub extern crate tower_service;
pub extern crate tower_util;

use bytes::{Bytes, Buf};
use tower_h2::{Body, RecvBody};
use futures::{Future, Poll, Async};

pub use h2_support::env_logger;

// We can't import `try_ready` here because this module isn't at the crate
// root, so we'll redefine it instead.
#[macro_export]
macro_rules! try_ready {
    ($e:expr) => (match $e {
        Ok(futures::Async::Ready(t)) => t,
        Ok(futures::Async::NotReady) =>
            return Ok(futures::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}


pub mod rt {
    use futures::Future;
    use tokio::runtime::current_thread::Runtime;
    use std::fmt;

    /// Helper function to construct a new single-threaded `Runtime`, spawn a
    /// "background" future, and then block on a "foreground" future.
    pub fn spawn_bg_and_run<F, B>(
        foreground: F,
        background: B,
    ) -> F::Item
    where
        F: Future,
        F::Error: fmt::Debug,
        B: Future<Item = ()> + 'static,
        B::Error: fmt::Debug,
    {
        let _ = super::env_logger::try_init();

        Runtime::new().expect("create runtime")
            .spawn(background
                .map_err(|e| panic!("background future error: {:?}", e)))
            .block_on(foreground)
            .expect("foreground future error")
    }
}


#[derive(Default)]
pub struct SendBody {
    body: Option<Bytes>,
    trailers: Option<http::HeaderMap>,
}

impl SendBody {
    pub fn new<I: Into<Bytes>>(body: I) -> Self {
        SendBody {
            body: Some(body.into()),
            ..Default::default()
        }
    }

    pub fn with_trailers(self, trailers: http::HeaderMap) -> Self {
        Self {
            trailers: Some(trailers),
            ..self
        }
    }
}

impl Body for SendBody {
    type Data = Bytes;

    fn is_end_stream(&self) -> bool {
        let body_is_eos = self.body.as_ref().map(|b| b.is_empty()).unwrap_or(true);
        body_is_eos && self.trailers.is_none()
    }

    fn poll_data(&mut self) -> Poll<Option<Bytes>, h2::Error> {
        let data = self.body
            .take()
            .and_then(|b| if b.is_empty() { None } else { Some(b) });
        Ok(Async::Ready(data))
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        Ok(Async::Ready(self.trailers.take()))
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
    type Error = self::h2::Error;
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

pub fn read_recv_body_and_trailers(body: RecvBody) -> ReadRecvBodyAndTrailers  {
    ReadRecvBodyAndTrailers {
        body,
        bytes: None,
        is_reading_trailers: false,
    }
}
pub struct ReadRecvBodyAndTrailers {
    body: RecvBody,
    bytes: Option<Box<Buf>>,
    is_reading_trailers: bool,
}

impl Future for ReadRecvBodyAndTrailers {
    type Item = (Option<Bytes>, Option<http::HeaderMap>);
    type Error = self::h2::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            if self.is_reading_trailers {
                let trailers = try_ready!(self.body.poll_trailers());
                let body = self.bytes.take().map(Buf::collect);
                return Ok(Async::Ready((body, trailers)));
            }
            match try_ready!(self.body.poll_data()) {
                None => {
                    self.is_reading_trailers = true;
                },
                Some(b) => {
                    self.bytes = if self.bytes.as_ref().is_none() {
                        Some(Box::new(b))
                    } else {
                        Some(Box::new(self.bytes.take().unwrap().chain(b)))
                    };
                },
            }
        }
    }
}

