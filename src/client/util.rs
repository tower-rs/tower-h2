use tower_service::Service;
use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};

pub trait ConnectService<A>{
    type Response: AsyncRead + AsyncWrite;
    type Error;
    type Future: Future<Item =  Self::Response, Error = Self::Error>;

    fn connect(&mut self, target: A) -> Self::Future;
}

impl<A, C> ConnectService<A> for C
where 
    C: Service<A>,
    C::Response: AsyncRead + AsyncWrite,
{
    type Response = C::Response;
    type Error = C::Error;
    type Future = C::Future;

    fn connect(&mut self, target: A) -> Self::Future {
        self.call(target)
    }
}