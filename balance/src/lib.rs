extern crate futures;
extern crate http;
extern crate h2;
extern crate tower_balance;
extern crate tower_h2;

use futures::{Async, Poll};
use std::marker::PhantomData;
use tower_balance::load::Instrument;

/// Instruments HTTP responses to drop handles when their first body message is received.
#[derive(Clone, Debug)]
pub struct PendingUntilFirstData;

/// Instruments HTTP responses to drop handles when their streams completes.
#[derive(Clone, Debug)]
pub struct PendingUntilEos;

/// An instrumented HTTP body that drops its handle according to the `S`-typed strategy.
pub struct Body<T, B, S> {
    handle: Option<T>,
    body: B,
    _p: PhantomData<S>,
}

fn instrument_body<T, B, S>(handle: T, rsp: http::Response<B>) -> http::Response<Body<T, B, S>> {
    let (parts, body) = rsp.into_parts();
    let body = Body {
        handle: Some(handle),
        body,
        _p: PhantomData
    };
    http::Response::from_parts(parts, body)
}

// ==== PendingUntilEos ====

impl<T, B> Instrument<T, http::Response<B>> for PendingUntilFirstData
where
    T: Sync + Send + 'static,
    B: tower_h2::Body + 'static,
{
    type Output = http::Response<Body<T, B, Self>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        instrument_body(handle, rsp)
    }
}

// ==== PendingUntilEos ====

impl<T, B> Instrument<T, http::Response<B>> for PendingUntilEos
where
    T: Sync + Send + 'static,
    B: tower_h2::Body + 'static,
{
    type Output = http::Response<Body<T, B, Self>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        instrument_body(handle, rsp)
    }
}

// ==== Body ====

/// Helps to ensure a future is not ready, regardless of whether it failed or not.
macro_rules! ready {
    ($poll:expr) => {
        match $poll {
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            ret => ret,
        }
    };
}

impl<T, B> tower_h2::Body for Body<T, B, PendingUntilFirstData>
where
    B: tower_h2::Body,
{
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let ret = ready!(self.body.poll_data());

        // Once a data frame is received, the handle is dropped. On subsequent calls, this
        // is a noop.
        drop(self.handle.take());

        ret
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        // If this is being called, the handle definitely should have been dropped
        // already.
        drop(self.handle.take());

        self.body.poll_trailers()
    }
}

impl<T, B: tower_h2::Body> tower_h2::Body for Body<T, B, PendingUntilEos> {
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let ret = ready!(self.body.poll_data());

        // If this was the last frame, then drop the handle immediately.
        if self.is_end_stream() {
            drop(self.handle.take());
        }

        ret
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        let ret = ready!(self.body.poll_trailers());

        // Once trailers are received, the handle is dropped immediately (in case the body
        // is retained longer for some reason).
        drop(self.handle.take());

        ret
    }
}
