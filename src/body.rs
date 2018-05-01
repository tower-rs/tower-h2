use h2;
use bytes::{IntoBuf, Bytes};
use futures::{Async, Poll};
use http::HeaderMap;

use std::fmt;

/// A generic h2 client/server request/response body.
pub trait Body {
    /// The body chunk type
    type Data: IntoBuf + 'static;

    /// Returns `true` when the end of stream has been reached.
    ///
    /// An end of stream means that both `poll_data` and `poll_trailers` will
    /// return `None`.
    ///
    /// A return value of `false` **does not** guarantee that a value will be
    /// returend from `poll_stream` or `poll_trailers`.
    fn is_end_stream(&self) -> bool {
        false
    }

    /// Polls a stream of data.
    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error>;

    /// Returns possibly **one** `HeaderMap` for trailers.
    fn poll_trailers(&mut self) -> Poll<Option<HeaderMap>, h2::Error> {
        Ok(Async::Ready(None))
    }
}

/// Dynamic `Send` body object.
pub struct BoxBody<T = Bytes> {
    inner: Box<Body<Data = T> + Send + 'static>,
}

/// Dynamic `!Send` body object.
pub struct UnsyncBoxBody<T = Bytes> {
    inner: Box<Body<Data = T> + 'static>,
}

// ===== impl Body =====

impl Body for () {
    type Data = &'static [u8];

    #[inline]
    fn is_end_stream(&self) -> bool {
        true
    }

    #[inline]
    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        Ok(Async::Ready(None))
    }

    #[inline]
    fn poll_trailers(&mut self) -> Poll<Option<HeaderMap>, h2::Error> {
        Ok(Async::Ready(None))
    }
}

// ===== impl BoxBody =====

impl<T> BoxBody<T> {
    /// Create a new `BoxBody` backed by `inner`.
    pub fn new(inner: Box<Body<Data = T> + Send + 'static>) -> Self {
        BoxBody { inner }
    }
}

impl<T> Body for BoxBody<T>
where T: IntoBuf + 'static,
{
    type Data = T;

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        self.inner.poll_data()
    }

    fn poll_trailers(&mut self) -> Poll<Option<HeaderMap>, h2::Error> {
        self.inner.poll_trailers()
    }
}

impl<T: fmt::Debug> fmt::Debug for BoxBody<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxBody")
            .finish()
    }
}

// ===== impl UnsyncBoxBody =====

impl<T> UnsyncBoxBody<T> {
    /// Create a new `UnsyncBoxBody` backed by `inner`.
    pub fn new(inner: Box<Body<Data = T> + 'static>) -> Self {
        UnsyncBoxBody { inner }
    }
}

impl<T> Body for UnsyncBoxBody<T>
where T: IntoBuf + 'static,
{
    type Data = T;

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        self.inner.poll_data()
    }

    fn poll_trailers(&mut self) -> Poll<Option<HeaderMap>, h2::Error> {
        self.inner.poll_trailers()
    }
}

impl<T: fmt::Debug> fmt::Debug for UnsyncBoxBody<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxBody")
            .finish()
    }
}
