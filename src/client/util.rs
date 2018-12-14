use tower_service::Service;
use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};

pub trait ConnectService<A>{
    type Response: AsyncRead + AsyncWrite;
    type Error;
    type Future: Future<Item =  Self::Response, Error = Self::Error>;

    fn connect(&self) -> Self::Future;
}

impl<A, C> ConnectService<A> for C
    where C: Service<A>,
          C::Response: AsyncRead + AsyncWrite,
{
    type Response = C::Response;
    type Error = C::Error;
    type Future = C::Future;

    // TODO: don't quite know why I need to suppress this, doesn't seem
    // like it's needed for MakeService. 
    #[allow(unconditional_recursion)]
    fn connect(&self) -> Self::Future {
        ConnectService::connect(self)
    }
}