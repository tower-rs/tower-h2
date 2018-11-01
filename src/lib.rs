extern crate bytes;
#[macro_use]
extern crate futures;
extern crate h2;
extern crate http;
#[macro_use]
extern crate log;
extern crate tokio_connect;
extern crate tokio_io;
extern crate tower_service;

pub mod client;
pub mod server;

mod body;
mod buf;
mod flush;
mod recv_body;
mod service;

pub use body::{Body, BoxBody, UnsyncBoxBody};
pub use recv_body::{RecvBody, Data};
pub use server::Server;
pub use service::HttpService;

mod sealed {
    /// Private trait to this crate to prevent traits from being implemented in
    /// downstream crates.
    pub trait Sealed {}
    /// Like `Sealed` but for types such as `HttpService` which would otherwise
    /// have unconstrained type parameters in blanket impls of Sealed.
    pub trait GenericSealed1<A> {}
}
