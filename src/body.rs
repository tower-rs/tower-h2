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

mod test {
    use super::*;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

    struct MockBody {
        is_end_stream: bool,
        is_end_stream_calls: AtomicUsize,
        poll_data_calls: AtomicUsize,
        poll_trailers_calls: AtomicUsize,
    }

    impl MockBody {
        fn new() -> Self {
            Self {
                is_end_stream: false,
                is_end_stream_calls: AtomicUsize::new(0),
                poll_data_calls: AtomicUsize::new(0),
                poll_trailers_calls: AtomicUsize::new(0),
            }
        }
    }

    impl Body for Arc<MockBody> {
        type Data = &'static [u8];

        fn is_end_stream(&self) -> bool {
            self.is_end_stream_calls.fetch_add(1, Ordering::Relaxed);
            self.is_end_stream
        }

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            self.poll_data_calls.fetch_add(1, Ordering::Relaxed);
            Ok(Async::Ready(None))
        }

        fn poll_trailers(&mut self) -> Poll<Option<HeaderMap>, h2::Error> {
            self.poll_trailers_calls.fetch_add(1, Ordering::Relaxed);
            Ok(Async::Ready(None))
        }
    }

    #[test]
    fn box_body_proxies_to_inner() {
        let mock = Arc::new(MockBody::new());
        let mut body = BoxBody::new(Box::new(mock.clone()));

        assert_eq!(body.is_end_stream(), false);
        assert_eq!(mock.is_end_stream_calls.load(Ordering::Relaxed), 1);

        assert_eq!(body.poll_data().unwrap(), Async::Ready(None));
        assert_eq!(mock.poll_data_calls.load(Ordering::Relaxed), 1);

        assert_eq!(body.poll_trailers().unwrap(), Async::Ready(None));
        assert_eq!(mock.poll_trailers_calls.load(Ordering::Relaxed), 1);
    }


    #[test]
    fn unsync_box_body_proxies_to_inner() {
        let mock = Arc::new(MockBody::new());
        let mut body = UnsyncBoxBody::new(Box::new(mock.clone()));

        assert_eq!(body.is_end_stream(), false);
        assert_eq!(mock.is_end_stream_calls.load(Ordering::Relaxed), 1);

        assert_eq!(body.poll_data().unwrap(), Async::Ready(None));
        assert_eq!(mock.poll_data_calls.load(Ordering::Relaxed), 1);

        assert_eq!(body.poll_trailers().unwrap(), Async::Ready(None));
        assert_eq!(mock.poll_trailers_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn unit_body() {
        let mut body = ();

        assert_eq!(body.is_end_stream(), true);
        assert_eq!(body.poll_data().unwrap(), Async::Ready(None));
        assert_eq!(body.poll_trailers().unwrap(), Async::Ready(None));
    }
}
