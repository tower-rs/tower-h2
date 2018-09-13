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
