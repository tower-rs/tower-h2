use Body;
use bytes::{Bytes, BytesMut, Buf};
use futures::{Poll, Stream};
use h2;
use http;

/// Allows a stream to be read from the remote.
#[derive(Debug)]
pub struct RecvBody {
    inner: h2::RecvStream,
}

#[derive(Debug)]
pub struct Data {
    bytes: Bytes,
}

// ===== impl RecvBody =====

impl RecvBody {
    /// Return a new `RecvBody`.
    pub(crate) fn new(inner: h2::RecvStream) -> Self {
        RecvBody { inner }
    }

    /// Returns the stream ID of the received stream, or `None` if this body
    /// does not correspond to a stream.
    pub fn stream_id(&self) -> h2::StreamId {
        self.inner.stream_id()
    }
}

impl Body for RecvBody {
    type Item = Data;
    type Error = h2::Error;

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn poll_buf(&mut self) -> Poll<Option<Self::Item>, h2::Error> {
        let data = try_ready!(self.inner.poll())
            .map(|bytes| {
                self.inner
                    .release_capacity()
                    .release_capacity(bytes.len())
                    .expect("flow control error");
                Data {
                    bytes,
                }
            });

        Ok(data.into())
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        self.inner.poll_trailers()
    }
}

// ===== impl Data =====

impl Buf for Data {
    fn remaining(&self) -> usize {
        self.bytes.len()
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    fn advance(&mut self, cnt: usize) {
        self.bytes.advance(cnt);
    }
}

impl From<Data> for Bytes {
    fn from(src: Data) -> Self {
        src.bytes
    }
}

impl From<Data> for BytesMut {
    fn from(src: Data) -> Self {
        src.bytes.into()
    }
}
