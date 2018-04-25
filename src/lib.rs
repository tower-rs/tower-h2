extern crate bytes;
#[macro_use]
extern crate futures;
extern crate h2;
extern crate http;
#[macro_use]
extern crate log;
extern crate tokio_connect;
extern crate tokio_core;
extern crate tokio_io;
extern crate tower_service;

pub mod client;
pub mod server;

mod body;
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
}
