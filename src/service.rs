use Body;

use http::{Request, Response};
use futures::{Future, Poll};
use tower::Service;

/// An HTTP service
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts. Implements the `tower::Service` trait using `http::Request`
/// and `http::Response` types.
pub trait HttpService: ::sealed::Sealed {
    /// Request payload.
    type RequestBody: Body;

    /// Response payload.
    type ResponseBody: Body;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Item = Response<Self::ResponseBody>, Error = Self::Error>;

    /// Returns `Ready` when the service is able to process requests.
    fn poll_ready(&mut self) -> Poll<(), Self::Error>;

    /// Process the request and return the response asynchronously.
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
