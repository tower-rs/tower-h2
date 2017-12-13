use {Body, RecvBody};
use super::{Connection, Background};

use futures::{Future, Async, Poll};
use futures::future::Executor;
use h2;
use http::{Request, Response};
use tokio_connect;

use std::boxed::Box;
use std::marker::PhantomData;

/// Establishes an H2 client connection.
///
/// Has a builder-like API for configuring client connections.  Currently this only allows
/// the configuration of TLS transport on new services created by this factory.
pub struct Connect<C, E, S> {
    /// Establish new session layer values (usually TCP sockets w/ TLS).
    inner: C,

    /// H2 client configuration
    builder: h2::client::Builder,

    /// Used to spawn connection management tasks and tasks to flush send
    /// body streams.
    executor: E,

    /// The HTTP request body type.
    _p: PhantomData<S>,
}

/// Completes with a Connection when the H2 connection has been initialized.
pub struct ConnectFuture<C, E, S>
where C: tokio_connect::Connect + 'static,
      S: Body + 'static,
{
    future: Box<Future<Item = Connected<S::Data, C::Connected>, Error = ConnectError<C::Error>>>,
    executor: E,
    _p: PhantomData<S>,
}

/// The type yielded by an h2 client handshake future
type Connected<S, C> = (h2::client::Client<S>, h2::client::Connection<C, S>);

/// Error produced when establishing an H2 client connection.
#[derive(Debug)]
pub enum ConnectError<T> {
    /// An error occurred when attempting to establish the underlying session
    /// layer.
    Connect(T),

    /// An error occurred when attempting to perform the HTTP/2.0 handshake.
    Proto(h2::Error),

    /// An error occured when attempting to execute a worker task
    Execute,
}

// ===== impl Connect =====

impl<C, E, S> Connect<C, E, S>
where
    C: tokio_connect::Connect,
    E: Executor<Background<C, S>> + Clone,
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
    E: Executor<Background<C, S>> + Clone,
    S: Body + 'static,
{
    type Request = Request<S>;
    type Response = Response<RecvBody>;
    type Error = super::Error;
    type InitError = ConnectError<C::Error>;
    type Service = Connection<C, E, S>;
    type Future = ConnectFuture<C, E, S>;

    /// Obtains a Connection on a single plaintext h2 connection to a remote.
    fn new_service(&self) -> Self::Future {
        let client = self.builder.clone();
        let conn = self.inner.connect()
            .map_err(ConnectError::Connect)
            .and_then(move |io| {
                client
                    .handshake(io)
                    .map_err(ConnectError::Proto)
            });

        ConnectFuture {
            future: Box::new(conn),
            executor: self.executor.clone(),
            _p: PhantomData,
        }
    }
}

// ===== impl ConnectFuture =====

impl<C, E, S> Future for ConnectFuture<C, E, S>
where
    C: tokio_connect::Connect,
    E: Executor<Background<C, S>> + Clone,
    S: Body,
{
    type Item = Connection<C, E, S>;
    type Error = ConnectError<C::Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // Get the session layer instance
        let (client, connection) = try_ready!(self.future.poll());

        // Spawn the worker task
        let task = Background::connection(connection);
        self.executor.execute(task).map_err(|_| ConnectError::Execute)?;

        // Create an instance of the service
        let service = Connection::new(client, self.executor.clone());

        Ok(Async::Ready(service))
    }
}
