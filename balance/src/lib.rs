extern crate futures;
extern crate h2;
extern crate http;
extern crate tower_balance;
extern crate tower_h2;

use futures::{Async, Poll};
use tower_balance::load::Instrument;

/// Instruments HTTP responses to drop handles when their first body message is received.
#[derive(Clone, Debug, Default)]
pub struct PendingUntilFirstData(());

/// Instruments HTTP responses to drop handles when their streams completes.
#[derive(Clone, Debug, Default)]
pub struct PendingUntilEos(());

/// An instrumented HTTP body that drops its handle when the first data is received.
pub struct PendingUntilFirstDataBody<T, B> {
    handle: Option<T>,
    body: B,
}

/// An instrumented HTTP body that drops its handle upon completion.
pub struct PendingUntilEosBody<T, B> {
    handle: Option<T>,
    body: B,
}

// ==== PendingUntilEos ====::default()

impl<T, B> Instrument<T, http::Response<B>> for PendingUntilFirstData
where
    T: Sync + Send + 'static,
    B: tower_h2::Body + 'static,
{
    type Output = http::Response<PendingUntilFirstDataBody<T, B>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        let (parts, body) = rsp.into_parts();
        let handle = if body.is_end_stream() {
            None
        } else {
            Some(handle)
        };
        let body = PendingUntilFirstDataBody { handle, body };
        http::Response::from_parts(parts, body)
    }
}

// ==== PendingUntilEos ====::default()

impl<T, B> Instrument<T, http::Response<B>> for PendingUntilEos
where
    T: Sync + Send + 'static,
    B: tower_h2::Body + 'static,
{
    type Output = http::Response<PendingUntilEosBody<T, B>>;

    fn instrument(&self, handle: T, rsp: http::Response<B>) -> Self::Output {
        let (parts, body) = rsp.into_parts();
        let handle = if body.is_end_stream() {
            None
        } else {
            Some(handle)
        };
        let body = PendingUntilEosBody { handle, body };
        http::Response::from_parts(parts, body)
    }
}

// ==== Body ====

/// Helps to ensure a future is not ready, regardless of whether it failed or not.
macro_rules! return_if_not_ready {
    ($poll:expr) => {
        match $poll {
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            ret => ret,
        }
    };
}

impl<T, B> tower_h2::Body for PendingUntilFirstDataBody<T, B>
where
    B: tower_h2::Body,
{
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let ret = return_if_not_ready!(self.body.poll_data());

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

impl<T, B: tower_h2::Body> tower_h2::Body for PendingUntilEosBody<T, B> {
    type Data = B::Data;

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        let ret = return_if_not_ready!(self.body.poll_data());

        // If this was the last frame, then drop the handle immediately.
        if self.is_end_stream() {
            drop(self.handle.take());
        }

        ret
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        let ret = return_if_not_ready!(self.body.poll_trailers());

        // Once trailers are received, the handle is dropped immediately (in case the body
        // is retained longer for some reason).
        drop(self.handle.take());

        ret
    }
}

#[cfg(test)]
mod tests {
    use futures::Poll;
    use h2;
    use http;
    use std::collections::VecDeque;
    use std::sync::{Arc, Weak};
    use tower_balance::load::Instrument;
    use tower_h2::Body;

    use super::{PendingUntilFirstData, PendingUntilEos};

    #[test]
    fn first_data() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            TestBody(parts, None)
        };

        let (h, wk) = Handle::new();
        let (_, mut body) = PendingUntilFirstData::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn first_data_empty() {
        let body = TestBody::default();

        let (h, wk) = Handle::new();
        let (_, _body) = PendingUntilFirstData::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn first_data_drop() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            TestBody(parts, None)
        };

        let (h, wk) = Handle::new();
        let (_, body) = PendingUntilFirstData::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        drop(body);
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn first_data_error() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            parts.push_back("two");
            let e: ::std::io::Error = ::std::io::ErrorKind::Other.into();
            ErrBody(Some(e.into()))
        };

        let (h, wk) = Handle::new();
        let (_, mut body) = PendingUntilFirstData::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().is_err());
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn eos() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            parts.push_back("two");
            TestBody(parts, None)
        };

        let (h, wk) = Handle::new();
        let (_, mut body) = PendingUntilEos::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn eos_empty() {
        let body = TestBody::default();

        let (h, wk) = Handle::new();
        let (_, _body) = PendingUntilEos::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn eos_trailers() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            parts.push_back("two");
            TestBody(parts, Some(http::HeaderMap::default()))
        };

        let (h, wk) = Handle::new();
        let (_, mut body) = PendingUntilEos::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().expect("data").is_ready());
        assert!(wk.upgrade().is_some());

        assert!(body.poll_trailers().expect("trailers").is_ready());
        assert!(wk.upgrade().is_none());
    }

    #[test]
    fn eos_error() {
        let body = {
            let mut parts = VecDeque::new();
            parts.push_back("one");
            parts.push_back("two");
            let e: ::std::io::Error = ::std::io::ErrorKind::Other.into();
            ErrBody(Some(e.into()))
        };

        let (h, wk) = Handle::new();
        let (_, mut body) = PendingUntilEos::default()
            .instrument(h, http::Response::new(body))
            .into_parts();
        assert!(wk.upgrade().is_some());

        assert!(body.poll_data().is_err());
        assert!(wk.upgrade().is_none());
    }

    struct Handle(Arc<()>);
    impl Handle {
        fn new() -> (Self, Weak<()>) {
            let strong = Arc::new(());
            let weak = Arc::downgrade(&strong);
            (Handle(strong), weak)
        }
    }

    #[derive(Default)]
    struct TestBody(VecDeque<&'static str>, Option<http::HeaderMap>);
    impl Body for TestBody {
        type Data = &'static str;

        fn is_end_stream(&self) -> bool {
            self.0.is_empty() & self.1.is_none()
        }

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            Ok(self.0.pop_front().into())
        }

        fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
            assert!(self.0.is_empty());
            Ok(self.1.take().into())
        }
    }

    #[derive(Default)]
    struct ErrBody(Option<h2::Error>);
    impl Body for ErrBody {
        type Data = &'static str;

        fn is_end_stream(&self) -> bool {
            self.0.is_none()
        }

        fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
            Err(self.0.take().expect("err"))
        }

        fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
            Err(self.0.take().expect("err"))
        }
    }
}
