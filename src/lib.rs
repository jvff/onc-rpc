extern crate bytes;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;
extern crate tokio_io;

mod errors;
mod message;
mod record;
