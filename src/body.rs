use bytes::Buf;
use futures::Poll;
use Body;

#[derive(Debug, Default)]
pub struct NoBody;

#[derive(Debug, Default)]
pub struct NoData;

impl Body for NoBody {
    type Data = NoData;
    type Error = h2::Error;

    fn is_end_stream(&self) -> bool {
        true
    }

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, h2::Error> {
        Ok(None.into())
    }

    fn poll_trailers(&mut self) -> Poll<Option<http::HeaderMap>, h2::Error> {
        Ok(None.into())
    }
}

impl Buf for NoData {
    fn remaining(&self) -> usize {
        0
    }

    fn bytes(&self) -> &[u8] {
        &[]
    }

    fn advance(&mut self, _cnt: usize) {}
}
