pub extern crate bytes;
pub extern crate futures;
pub extern crate h2;
pub extern crate h2_support;
pub extern crate http;
pub extern crate tokio;
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

pub struct SendBody(Option<Bytes>);

impl SendBody {
    pub fn new<I: Into<Bytes>>(body: I) -> Self {
        SendBody(Some(body.into()))
    }

    pub fn empty() -> Self {
        SendBody(None)
    }
}

impl Body for SendBody {
    type Data = Bytes;

    fn is_end_stream(&self) -> bool {
        self.0.as_ref().map(|b| b.is_empty()).unwrap_or(true)
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
