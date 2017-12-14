use Body;
use flush::Flush;

use futures::{Future, Poll};
use h2::client::Connection;
use tokio_io::{AsyncRead, AsyncWrite};

/// Task that performs background tasks for a client.
///
/// This is not used directly by a user of this library.
pub struct Background<T, S>
where S: Body,
{
    task: Task<T, S>,
}

/// The specific task to execute
enum Task<T, S>
where S: Body,
{
    Connection(Connection<T, S::Data>),
    Flush(Flush<S>),
}

// ===== impl Background =====

impl<T, S> Background<T, S>
where S: Body,
{
    pub(crate) fn connection(connection: Connection<T, S::Data>) -> Self {
        let task = Task::Connection(connection);
        Background { task }
    }

    pub(crate) fn flush(flush: Flush<S>) -> Self {
        let task = Task::Flush(flush);
        Background { task }
    }
}

impl<T, S> Future for Background<T, S>
where T: AsyncRead + AsyncWrite,
      S: Body,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Task::*;

        match self.task {
            // TODO: Log error?
            Connection(ref mut f) => f.poll().map_err(|_| ()),
            Flush(ref mut f) => f.poll(),
        }
    }
}
