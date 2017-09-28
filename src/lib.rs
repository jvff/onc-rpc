extern crate byteorder;
extern crate bytes;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod errors;
#[macro_use]
mod macros;
mod message;
mod record;
mod rpc;
mod service;

pub mod port_mapper;

pub use self::errors::{Error, ErrorKind, Result};
pub use self::message::RpcMessage;
pub use self::record::{Record, RecordProtocol};
pub use self::rpc::{RpcCall, RpcProcedure, RpcProgram};
pub use self::service::{CallFuture, DeserializeWithHint, RpcRequest, RpcService,
                        RpcServiceConfig, TryFrom};
