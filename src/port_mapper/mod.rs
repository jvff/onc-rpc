//! Port Mapper remote program client and server.
//!
//! The [Port Mapper program][rfc] runs on an RPC server and is responsible for
//! mapping programs available to clients to the ports they are available on.
//! This module defines the remote program interface ([`PortMapper`][trait]) and
//! provides both synchronous ([`PortMapperSyncClient`][sync-client]) and
//! asynchronous ([`PortMapperAsyncClient`][async-client]) client
//! implementations and a simple server ([`PortMapperServer`][server])
//! implementation.
//!
//! [async-client]: struct.PortMapperAsyncClient.html
//! [rfc]: https://tools.ietf.org/html/rfc1057#page-22
//! [server]: struct.PortMapperServer.html
//! [sync-client]: struct.PortMapperSyncClient.html
//! [trait]: trait.PortMapper.html
//! ```

mod call_args;
mod call_result;
mod mapping;
mod protocol;

mod port_mapper;
mod port_mapper_server;

mod hash_map_port_mapper;

pub use self::call_args::CallArgs;
pub use self::call_result::CallResult;
pub use self::mapping::Mapping;
pub use self::protocol::Protocol;

pub use self::port_mapper::{GetPortResult, PortMapper, PortMapperAsyncClient,
                            PortMapperConnect, PortMapperServerWrapper,
                            PortMapperSyncClient};
pub use self::port_mapper_server::PortMapperServer;
