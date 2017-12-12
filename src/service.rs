use Body;

use http::{Request, Response};
use futures::{Future, Poll};
use tower::Service;

/// An HTTP (2.0) service that backs the gRPC client
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts. Implement the `tower::Service` trait using `http::Request`
/// and `http::Response` types.
pub trait HttpService: ::sealed::Sealed {
    type RequestBody: Body;
    type ResponseBody: Body;
    type Error;
    type Future: Future<Item = Response<Self::ResponseBody>, Error = Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error>;

    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future;
}

impl<T, B1, B2> HttpService for T
where T: Service<Request = Request<B1>,
                Response = Response<B2>>,
      B1: Body,
      B2: Body,
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Service::poll_ready(self)
    }

    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future {
        Service::call(self, request)
    }
}

impl<T, B1, B2> ::sealed::Sealed for T
where T: Service<Request = Request<B1>,
                Response = Response<B2>>,
      B1: Body,
      B2: Body,
{}
