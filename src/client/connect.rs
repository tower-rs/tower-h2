use {Body, RecvBody};
use super::{Connection, Background, Handshake, HandshakeError};

use futures::{Future, Poll};
use futures::future::Executor;
use h2;
use http::{Request, Response};
use tokio_connect;

use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

/// Establishes an H2 client connection.
///
/// Has a builder-like API for configuring client connections.  Currently this only allows
/// the configuration of TLS transport on new services created by this factory.
pub struct Connect<C, E, S> {
    /// Establish new session layer values (usually TCP sockets w/ TLS).
    inner: C,

    /// HTTP/2.0 client configuration
    builder: h2::client::Builder,

    /// Used to spawn connection management tasks and tasks to flush send
    /// body streams.
    executor: E,

    /// The HTTP request body type.
    _p: PhantomData<S>,
}

/// Completes with a Connection when the H2 connection has been initialized.
pub struct ConnectFuture<C, E, S>
where C: tokio_connect::Connect,
      S: Body,
{
    /// Connect state. Starts in "Connect", which attempts to obtain the `io`
    /// handle from the `tokio_connect::Connect` instance. Then, with the
    /// handle, performs the HTTP/2.0 handshake.
    state: State<C, E, S>,

    /// The executor that the `Connection` will use to spawn request body stream
    /// flushing tasks
    executor: Option<E>,

    /// HTTP/2.0 client configuration
    builder: h2::client::Builder,
}

/// Represents the state of a `ConnectFuture`
enum State<C, E, S>
where C: tokio_connect::Connect,
      S: Body,
{
    Connect(C::Future),
    Handshake(Handshake<C::Connected, E, S>),
}

/// Error produced when establishing an H2 client connection.
#[derive(Debug)]
pub enum ConnectError<T> {
    /// An error occurred when attempting to establish the underlying session
    /// layer.
    Connect(T),

    /// An error occurred while performing the HTTP/2.0 handshake.
    Handshake(HandshakeError),
}

// ===== impl Connect =====

impl<C, E, S> Connect<C, E, S>
where
    C: tokio_connect::Connect,
    E: Executor<Background<C::Connected, S>> + Clone,
    S: Body,
{
    /// Create a new `Connect`.
    ///
    /// The `connect` argument is used to obtain new session layer instances
    /// (`AsyncRead` + `AsyncWrite`). For each new client service returned, a
    /// task will be spawned onto `executor` that will be used to manage the H2
    /// connection.
    pub fn new(inner: C, builder: h2::client::Builder, executor: E) -> Self {
        Connect {
            inner,
            executor,
            builder,
            _p: PhantomData,
        }
    }
}

impl<C, E, S> ::tower::NewService for Connect<C, E, S>
where
    C: tokio_connect::Connect + 'static,
    E: Executor<Background<C::Connected, S>> + Clone,
    S: Body + 'static,
{
    type Request = Request<S>;
    type Response = Response<RecvBody>;
    type Error = super::Error;
    type InitError = ConnectError<C::Error>;
    type Service = Connection<C::Connected, E, S>;
    type Future = ConnectFuture<C, E, S>;

    /// Obtains a Connection on a single plaintext h2 connection to a remote.
    fn new_service(&self) -> Self::Future {
        let state = State::Connect(self.inner.connect());
        let builder = self.builder.clone();

        ConnectFuture {
            state,
            builder,
            executor: Some(self.executor.clone()),
        }
    }
}

// ===== impl ConnectFuture =====

impl<C, E, S> Future for ConnectFuture<C, E, S>
where
    C: tokio_connect::Connect,
    E: Executor<Background<C::Connected, S>> + Clone,
    S: Body,
{
    type Item = Connection<C::Connected, E, S>;
    type Error = ConnectError<C::Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let io = match self.state {
                State::Connect(ref mut fut) => {
                    let res = fut.poll()
                        .map_err(ConnectError::Connect);

                    try_ready!(res)
                }
                State::Handshake(ref mut fut) => {
                    return fut.poll()
                        .map_err(ConnectError::Handshake);
                }
            };

            let executor = self.executor.take().expect("double poll");
            let handshake = Handshake::new(io, executor, &self.builder);

            self.state = State::Handshake(handshake);
        }
    }
}

// ===== impl ConnectError =====

impl<T> fmt::Display for ConnectError<T> where T: Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.description(), self.cause())
    }
}

impl<T> Error for ConnectError<T>
where
    T: Error,
{
    fn description(&self) -> &str {
        match *self {
            ConnectError::Connect(_) => 
                "Error while attempting to establish underlying session layer",
            ConnectError::Handshake(_) => 
                "Error while performing HTTP/2.0 handshake"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectError::Connect(ref why) => Some(why),
            ConnectError::Handshake(ref why) => Some(why),
        }
    }
}
