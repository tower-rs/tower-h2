use Body;
use super::{Connection, Background, Handshake, HandshakeError};

use tower_service::Service;
use tower::MakeConnection;

use futures::{Future, Poll};
use futures::future::Executor;
use h2;

use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

/// Establishes an H2 client connection.
///
/// Has a builder-like API for configuring client connections.  Currently this only allows
/// the configuration of TLS transport on new services created by this factory.
pub struct Connect<A, C, E, S> {
    /// Establish new session layer values (usually TCP sockets w/ TLS).
    inner: C,

    /// HTTP/2.0 client configuration
    builder: h2::client::Builder,

    /// Used to spawn connection management tasks and tasks to flush send
    /// body streams.
    executor: E,

    /// The HTTP request body type.
    _p: PhantomData<(A, S)>,
}

/// Completes with a Connection when the H2 connection has been initialized.
pub struct ConnectFuture<A, C, E, S>
where C: MakeConnection<A>,
      S: Body,
{
    /// Connect state. Starts in "Connect", which attempts to obtain the `io`
    /// handle from the `tokio_connect::Connect` instance. Then, with the
    /// handle, performs the HTTP/2.0 handshake.
    state: State<A, C, E, S>,

    /// The executor that the `Connection` will use to spawn request body stream
    /// flushing tasks
    executor: Option<E>,

    /// HTTP/2.0 client configuration
    builder: h2::client::Builder,
}

/// Represents the state of a `ConnectFuture`
enum State<A, C, E, S>
where C: MakeConnection<A>,
      S: Body,
{
    Connect(C::Future),
    Handshake(Handshake<C::Connection, E, S>),
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

impl<A, C, E, S> Connect<A, C, E, S>
where
    C: MakeConnection<A>,
    E: Executor<Background<C::Connection, S>> + Clone,
    S: Body,
    S::Item: 'static,
    S::Error: Into<Box<dyn std::error::Error>>,
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

impl<A, C, E, S> Service<A> for Connect<A, C, E, S>
where
    C: MakeConnection<A> + 'static,
    E: Executor<Background<C::Connection, S>> + Clone,
    S: Body + 'static,
    S::Error: Into<Box<dyn std::error::Error>>,
{
    type Response = Connection<C::Connection, E, S>;
    type Error = ConnectError<C::Error>;
    type Future = ConnectFuture<A, C, E, S>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready().map_err(ConnectError::Connect)
    }

    /// Obtains a Connection on a single plaintext h2 connection to a remote.
    fn call(&mut self, target: A) -> Self::Future {
        let state = State::Connect(self.inner.make_connection(target));
        let builder = self.builder.clone();

        ConnectFuture {
            state,
            builder,
            executor: Some(self.executor.clone()),
        }
    }
}

// ===== impl ConnectFuture =====

impl<A, C, E, S> Future for ConnectFuture<A, C, E, S>
where
    C: MakeConnection<A>,
    E: Executor<Background<C::Connection, S>> + Clone,
    S: Body,
    S::Item: 'static,
    S::Error: Into<Box<dyn std::error::Error>>,
{
    type Item = Connection<C::Connection, E, S>;
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

impl<T> fmt::Display for ConnectError<T>
where
    T: Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectError::Connect(ref why) => write!(f,
                "Error attempting to establish underlying session layer: {}",
                why
            ),
            ConnectError::Handshake(ref why) =>  write!(f,
                "Error while performing HTTP/2.0 handshake: {}",
                why,
            ),
        }
    }
}

impl<T> Error for ConnectError<T>
where
    T: Error,
{
    fn description(&self) -> &str {
        match *self {
            ConnectError::Connect(_) =>
                "error attempting to establish underlying session layer",
            ConnectError::Handshake(_) =>
                "error performing HTTP/2.0 handshake"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectError::Connect(ref why) => Some(why),
            ConnectError::Handshake(ref why) => Some(why),
        }
    }
}
