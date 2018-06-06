#[macro_use]
extern crate futures;
extern crate http;
extern crate h2;
extern crate tower_balance;
extern crate tower_h2;

use futures::Poll;
use tower_balance::load::Instrument;
use tower_h2::Body;

pub struct InstrumentFirstData;

pub struct InstrumentEos;

pub struct InstrumentFirstDataBody<T, B> {
    handle: Option<T>,
    body: B,
}

pub struct InstrumentEosBody<T, B> {
    handle: Option<T>,
    body: B,
}

impl<T, B> Instrument<T, http::Response<B>> for InstrumentFirstData
where
    T: Sync + Send + 'static,
    B: Body + 'static,
{
    type Output = http::Response<InstrumentFirstDataBody<T, B>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        let (parts, body) = rsp.into_parts();
        http::Response::from_parts(parts, InstrumentFirstDataBody {
            handle: Some(handle),
            body,
        })
    }
}

impl<T, B> Instrument<T, http::Response<B>> for InstrumentEos
where
    T: Sync + Send + 'static,
    B: Body + 'static,
{
    type Output = http::Response<InstrumentEosBody<T, B>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        let (parts, body) = rsp.into_parts();
        http::Response::from_parts(parts, InstrumentEosBody {
            handle: Some(handle),
            body,
        })
    }
}

impl<T, B: Body> Body for InstrumentFirstDataBody<T, B> {
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let data = try_ready!(self.body.poll_data());
        drop(self.handle.take());
        Ok(data.into())
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        self.body.poll_trailers()
    }
}

impl<T, B: Body> Body for InstrumentEosBody<T, B> {
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    /// Polls a stream of data.
    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let data = try_ready!(self.body.poll_data());
        if data.is_none() {
            drop(self.instrument.take());
        }
        Ok(data.into())
    }

    /// Returns possibly **one** `HeaderMap` for trailers.
    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        let trls = try_ready!(self.body.poll_trailers());
        drop(self.instrument.take());
        Ok(trls.into())
    }
}
