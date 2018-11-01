use {Body, RecvBody};
use super::Background;
use buf::SendBuf;
use flush::Flush;

use bytes::IntoBuf;
use futures::{Future, Poll, Async};
use futures::future::Executor;
use h2;
use h2::client::{self, SendRequest, Builder};
use http::{self, Request, Response};
use tokio_io::{AsyncRead, AsyncWrite};
use tower_service::Service;

use std::{error, fmt};
use std::marker::PhantomData;

/// Exposes a request/response API on an h2 client connection..
pub struct Connection<T, E, S>
where S: Body,
{
    client: SendRequest<SendBuf<<S::Data as IntoBuf>::Buf>>,
    executor: E,
    _p: PhantomData<(T, S)>,
}

/// In progress HTTP/2.0 client handshake.
pub struct Handshake<T, E, S>
where S: Body,
{
    inner: h2::client::Handshake<T, SendBuf<<S::Data as IntoBuf>::Buf>>,
    executor: E,
}

/// Drives the sending of a request (and its body) until a response is received (i.e. the
/// initial HEADERS or RESET frames sent from the remote).
///
/// This is necessary because, for instance, the remote server may not respond until the
/// request body is fully sent.
pub struct ResponseFuture {
    inner: Inner,
}

/// ResponseFuture inner
enum Inner {
    /// Inner response future
    Inner(client::ResponseFuture),

    /// Failed to send the request
    Error(Option<Error>),
}

/// Errors produced by client `Connection` calls.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

/// Error produced when performing an HTTP/2.0 handshake.
#[derive(Debug)]
pub enum HandshakeError {
    /// An error occurred when attempting to perform the HTTP/2.0 handshake.
    Proto(h2::Error),

    /// An error occured when attempting to execute a worker task
    Execute,
}

#[derive(Debug)]
enum Kind {
    Inner(h2::Error),
    Spawn,
}

// ===== impl Connection =====

impl<T, E, S> Connection<T, E, S>
where S: Body,
      S::Data: IntoBuf + 'static,
      E: Executor<Background<T, S>>,
      T: AsyncRead + AsyncWrite,
{
    /// Builds Connection on an H2 client connection.
    pub(crate) fn new(client: SendRequest<SendBuf<<S::Data as IntoBuf>::Buf>>, executor: E)
        -> Self
    {
        let _p = PhantomData;

        Connection {
            client,
            executor,
            _p,
        }
    }

    /// Perform the HTTP/2.0 handshake, yielding a `Connection` on completion.
    pub fn handshake(io: T, executor: E) -> Handshake<T, E, S> {
        Handshake::new(io, executor, &Builder::default())
    }
}

impl<T, E, S> Clone for Connection<T, E, S>
where S: Body,
      E: Clone,
{
    fn clone(&self) -> Self {
        Connection {
            client: self.client.clone(),
            executor: self.executor.clone(),
            _p: PhantomData,
        }
    }
}

impl<T, E, S> Service<Request<S>> for Connection<T, E, S>
where S: Body + 'static,
      S::Data: IntoBuf + 'static,
      E: Executor<Background<T, S>>,
      T: AsyncRead + AsyncWrite,
{
    type Response = Response<RecvBody>;
    type Error = Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.client.poll_ready()
            .map_err(Into::into)
    }

    fn call(&mut self, request: Request<S>) -> Self::Future {
        trace!("request: {} {}", request.method(), request.uri());

        // Split the request from the body
        let (parts, body) = request.into_parts();
        let request = http::Request::from_parts(parts, ());

        // If there is no body, then there is no point spawning a task to flush
        // it.
        let end_of_stream = body.is_end_stream();

        // Initiate the H2 request
        let res = self.client.send_request(request, end_of_stream);

        let (response, send_body) = match res {
            Ok(success) => success,
            Err(e) => {
                let e = Error { kind: Kind::Inner(e) };
                let inner = Inner::Error(Some(e));
                return ResponseFuture { inner };
            }
        };

        if !end_of_stream {
            let flush = Flush::new(body, send_body);
            let res = self.executor.execute(Background::flush(flush));

            if let Err(_) = res {
                let e = Error { kind: Kind::Spawn };
                let inner = Inner::Error(Some(e));
                return ResponseFuture { inner };
            }
        }

        ResponseFuture { inner: Inner::Inner(response) }
    }
}

// ===== impl ResponseFuture =====

impl Future for ResponseFuture {
    type Item = Response<RecvBody>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Inner::*;

        match self.inner {
            Inner(ref mut fut) => {
                let response = try_ready!(fut.poll());

                let (parts, body) = response.into_parts();
                let body = RecvBody::new(body);

                Ok(Response::from_parts(parts, body).into())
            }
            Error(ref mut e) => {
                return Err(e.take().unwrap());
            }
        }
    }
}

impl ResponseFuture {
    /// Returns the stream ID of the response stream, or `None` if this future
    /// does not correspond to a stream.
    pub fn stream_id(&self) -> Option<h2::StreamId> {
        match self.inner {
            Inner::Inner(ref rsp) => Some(rsp.stream_id()),
            _ => None,
        }
    }
}

// ===== impl Handshake =====

impl<T, E, S> Handshake<T, E, S>
where T: AsyncRead + AsyncWrite,
      S: Body,
{
    /// Start an HTTP/2.0 handshake with the provided builder
    pub(crate) fn new(io: T, executor: E, builder: &Builder) -> Self {
        let inner = builder.handshake(io);

        Handshake {
            inner,
            executor,
        }
    }
}

impl<T, E, S> Future for Handshake<T, E, S>
where T: AsyncRead + AsyncWrite,
      E: Executor<Background<T, S>> + Clone,
      S: Body,
{
    type Item = Connection<T, E, S>;
    type Error = HandshakeError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (client, connection) = try_ready!(self.inner.poll());

        // Spawn the worker task
        let task = Background::connection(connection);
        self.executor.execute(task)
            .map_err(|err| {
                warn!("error handshaking: {:?}", err);
                HandshakeError::Execute
            })?;

        // Create an instance of the service
        let service = Connection::new(client, self.executor.clone());

        Ok(Async::Ready(service))
    }
}

// ===== impl Error =====

impl Error {
    pub fn reason(&self) -> Option<h2::Reason> {
        match self.kind {
            Kind::Inner(ref h2) => h2.reason(),
            _ => None,
        }
    }
}

impl From<h2::Error> for Error {
    fn from(src: h2::Error) -> Self {
        Error { kind: Kind::Inner(src) }
    }
}

impl From<h2::Reason> for Error {
    fn from(src: h2::Reason) -> Self {
        h2::Error::from(src).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Inner(ref h2) =>
                write!(f, "Error caused by underlying HTTP/2 error: {}", h2),
            Kind::Spawn =>
                write!(f, "Error spawning background task"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        if let Kind::Inner(ref h2) = self.kind {
            Some(h2)
        } else {
            None
        }
    }

    fn description(&self) -> &str {
        match self.kind {
            Kind::Inner(ref h2) => h2.description(),
            Kind::Spawn => "error spawning worker task"
        }
    }

}

// ===== impl HandshakeError =====

impl From<h2::Error> for HandshakeError {
    fn from(src: h2::Error) -> Self {
        HandshakeError::Proto(src)
    }
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandshakeError::Proto(ref h2) =>
                write!(f,
                    "An error occurred while attempting to perform the HTTP/2 \
                    handshake: {}",
                    h2),
            HandshakeError::Execute =>
                write!(f,
                    "An error occurred while attempting to execute a worker \
                     task."),
        }
    }
}

impl error::Error for HandshakeError {
    fn cause(&self) -> Option<&error::Error> {
        if let HandshakeError::Proto(ref h2) = *self {
            Some(h2)
        } else {
            None
        }
    }

    fn description(&self) -> &str {
        match *self {
            HandshakeError::Proto(_) =>
                "error attempting to perform HTTP/2 handshake",
            HandshakeError::Execute =>
                "error attempting to execute a worker task",
        }
    }
}
