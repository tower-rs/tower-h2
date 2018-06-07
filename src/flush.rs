use Body;
use buf::SendBuf;

use bytes::IntoBuf;
use futures::{Future, Poll, Async};
use h2::{self, SendStream};
use http::HeaderMap;

/// Flush a body to the HTTP/2.0 send stream
pub(crate) struct Flush<S>
where S: Body,
{
    h2: SendStream<SendBuf<<S::Data as IntoBuf>::Buf>>,
    body: S,
    state: FlushState,
}

enum FlushState {
    Data,
    Trailers,
    Done,
}

enum DataOrTrailers<B> {
    Data(B),
    Trailers(HeaderMap),
}

// ===== impl Flush =====

impl<S> Flush<S>
where S: Body,
{
    pub fn new(src: S, dst: SendStream<SendBuf<<S::Data as IntoBuf>::Buf>>)
        -> Self
    {
        Flush {
            h2: dst,
            body: src,
            state: FlushState::Data,
        }
    }

    /// Try to flush the body.
    fn poll_complete(&mut self) -> Poll<(), h2::Error> {
        use self::DataOrTrailers::*;

        loop {
            match try_ready!(self.poll_body()) {
                Some(Data(buf)) => {
                    let eos = self.body.is_end_stream();

                    self.h2.send_data(SendBuf::new(buf.into_buf()), eos)?;

                    if eos {
                        self.state = FlushState::Done;
                        return Ok(Async::Ready(()));
                    }
                }
                Some(Trailers(trailers)) => {
                    self.h2.send_trailers(trailers)?;
                    return Ok(Async::Ready(()));
                }
                None => {
                    // If this is hit, then an EOS was not reached via the other
                    // paths. So, we must send an empty data frame with EOS.
                    self.h2.send_data(SendBuf::none(), true)?;

                    return Ok(Async::Ready(()));
                }
            }
        }
    }

    /// Get the next message to write, either a data frame or trailers.
    fn poll_body(&mut self)
        -> Poll<Option<DataOrTrailers<S::Data>>, h2::Error>
    {
        loop {
            match self.state {
                FlushState::Data => {
                    // Before trying to poll the next chunk, we have to see if
                    // the h2 connection has capacity. We do this by requesting
                    // a single byte (since we don't know how big the next chunk
                    // will be.
                    self.h2.reserve_capacity(1);

                    if self.h2.capacity() == 0 {
                        // TODO: The loop should not be needed once
                        // carllerche/h2#270 is fixed.
                        loop {
                            match try_ready!(self.h2.poll_capacity()) {
                                Some(0) => {}
                                Some(_) => break,
                                None => {
                                    debug!("connection closed early");
                                    // The error shouldn't really matter at this
                                    // point as the peer has disconnected, the
                                    // error will be discarded anyway.
                                    return Err(h2::Reason::INTERNAL_ERROR.into());
                                }
                            }
                        }
                    } else {
                        // If there was capacity already assigned, then the
                        // stream state wasn't polled, but we should fail out
                        // if the stream has been reset, so we poll for that.
                        match self.h2.poll_reset()? {
                            Async::Ready(reason) => {
                                debug!(
                                    "stream received RST_STREAM while flushing: {:?}",
                                    reason,
                                );
                                return Err(reason.into());
                            },
                            Async::NotReady => {
                                // Stream hasn't been reset, so we can try
                                // to send data below. This task has been
                                // registered in case data isn't ready
                                // before we get a RST_STREAM.
                            }
                        }
                    }



                    if let Some(data) = try_ready!(self.body.poll_data()) {
                        return Ok(Async::Ready(Some(DataOrTrailers::Data(data))));
                    } else {
                        // Release all capacity back to the connection
                        self.h2.reserve_capacity(0);
                        self.state = FlushState::Trailers;
                    }
                }
                FlushState::Trailers => {
                    match self.h2.poll_reset()? {
                        Async::Ready(reason) => {
                            debug!(
                                "stream received RST_STREAM while flushing trailers: {:?}",
                                reason,
                            );
                            return Err(reason.into());
                        },
                        Async::NotReady => {
                            // Stream hasn't been reset, so we can try
                            // to send trailers below. This task has been
                            // registered in case they aren't ready
                            // before we get a RST_STREAM.
                        }
                    }
                    let trailers = try_ready!(self.body.poll_trailers());
                    self.state = FlushState::Done;
                    if let Some(trailers) = trailers {
                        return Ok(Async::Ready(Some(DataOrTrailers::Trailers(trailers))));
                    }
                }
                FlushState::Done => return Ok(Async::Ready(None)),
            }
        }
    }
}

impl<S> Future for Flush<S>
where S: Body,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        // TODO: Do something with the error
        self.poll_complete().map_err(|_| ())
    }
}
