use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::service::Connect;

onc_rpc! {
    program(port_mapper::PortMapper) {
        id = 100_000;
        version = 2;

        export {
            /// Asynchronous client connection to a remote port mapper.
            ///
            /// This is a more complex type that can be used to connect to a
            /// remote port mapper. Connection is estabilished asynchronously
            /// through a `Future` which returns a `PortMapperAsyncClient`
            /// instance. The methods of that instance can be used to call the
            /// port mapper's remote procedures, allowing registration and query
            /// of available programs on the remote server.
            ///
            /// All methods return a instance that implements
            /// `Future<T, onc_rpc::Error>`. This is because the request is also
            /// performed asynchronously, and the connection used to perform the
            /// call and retrieve the result may fail.
            ///
            /// # Examples
            ///
            /// To use the asynchronous client, you must connect to a port
            /// mapper on a remote server. The RFC specifies that the port
            /// mapper should run on port 111.
            ///
            /// Afterwards, care should be taken to order all operations by
            /// ordering all the returned `Futures` correctly.
            ///
            /// ```
            /// extern crate futures;
            /// extern crate onc_rpc;
            /// extern crate tokio_core;
            ///
            /// use futures::Future;
            /// use tokio_core::reactor::Core;
            ///
            /// use onc_rpc::port_mapper::PortMapperAsyncClient;
            /// use onc_rpc::port_mapper::PortMapper;
            /// use onc_rpc::port_mapper::{Mapping, Protocol};
            /// #
            /// #   use std::thread;
            /// #   use std::time::Duration;
            /// #
            /// #   use onc_rpc::port_mapper::PortMapperServer;
            /// #
            /// #   fn start_server() {
            /// #       let address = "0.0.0.0:111".parse().unwrap();
            /// #       let server = PortMapperServer::new(address);
            /// #
            /// #       thread::spawn(move || server.serve());
            /// #
            /// #       // Give the server some time to start
            /// #       thread::sleep(Duration::from_millis(100));
            /// #   }
            ///
            /// fn main() {
            ///     #   start_server();
            ///     #
            ///     // Use a reactor::Core as the event loop
            ///     let mut reactor = Core::new().unwrap();
            ///
            ///     // Connect to the local port mapper program
            ///     let address = "127.0.0.1:111".parse().unwrap();
            ///     let connect_to_port_mapper =
            ///         PortMapperAsyncClient::connect_to_known_port(
            ///             address,
            ///             &reactor.handle(),
            ///         );
            ///
            ///     // Parameter to register a running program
            ///     let new_program = Mapping {
            ///         // Program ID
            ///         program: 10123,
            ///         // Program version
            ///         version: 1,
            ///         // Protocol it accepts connections
            ///         protocol: Protocol::Tcp,
            ///         // The port it's listening on
            ///         port: 55300,
            ///     };
            ///
            ///     // Parameter to query the running program
            ///     let program_query = Mapping {
            ///         // ID, version and protocol of the program to search for
            ///         program: 10123,
            ///         version: 1,
            ///         protocol: Protocol::Tcp,
            ///         // Ignored by remote procedure
            ///         port: 0,
            ///     };
            ///
            ///     // We only have a PortMapperAsyncClient instance after the
            ///     // connection future has resolved
            ///     let connect_then_set_then_query =
            ///         connect_to_port_mapper.and_then(|port_mapper| {
            ///             let set_operation = port_mapper.set(new_program);
            ///
            ///             // Ensure that the query happens after the set
            ///             // operation has completed
            ///             set_operation.and_then(move |_| {
            ///                 port_mapper.get_port(program_query)
            ///             })
            ///         });
            ///
            ///     // Wait for operations to finish
            ///     let port =
            ///         reactor.run(connect_then_set_then_query).unwrap();
            ///
            ///     assert_eq!(port, 55300);
            /// }
            use AsyncClient as PortMapperAsyncClient;

            /// Synchronous client connection to a remote port mapper.
            ///
            /// This is a simple type that can be used to connect to a remote
            /// port mapper. After a connection is estabilished, the port
            /// mapper's remote procedures can be called, allowing registration
            /// and query of available programs on the remote server.
            ///
            /// All methods return a `Result<T, onc_rpc::Error>` because the
            /// connection used to perform the call and retrieve the result may
            /// fail.
            ///
            /// # Examples
            ///
            /// To use the synchronous client, you must connect to a port mapper
            /// on a remote server. The RFC specifies that the port mapper
            /// should run on port 111.
            ///
            /// ```
            /// use onc_rpc::port_mapper::PortMapperSyncClient;
            /// use onc_rpc::port_mapper::{Mapping, Protocol};
            /// #
            /// #   use std::thread;
            /// #   use std::time::Duration;
            /// #
            /// #   use onc_rpc::port_mapper::PortMapperServer;
            /// #
            /// #   fn start_server() {
            /// #       let address = "0.0.0.0:111".parse().unwrap();
            /// #       let server = PortMapperServer::new(address);
            /// #
            /// #       thread::spawn(move || server.serve());
            /// #
            /// #       // Give the server some time to start
            /// #       thread::sleep(Duration::from_millis(100));
            /// #   }
            ///
            /// fn main() {
            ///     #   start_server();
            ///     #
            ///     // Connect to the local port mapper program
            ///     let address = "127.0.0.1:111".parse().unwrap();
            ///     let mut port_mapper =
            ///         PortMapperSyncClient::connect_to_known_port(address)
            ///             .unwrap();
            ///
            ///     // Register a running program
            ///     let new_program = Mapping {
            ///         // Program ID
            ///         program: 10123,
            ///         // Program version
            ///         version: 1,
            ///         // Protocol it accepts connections
            ///         protocol: Protocol::Tcp,
            ///         // The port it's listening on
            ///         port: 55300,
            ///     };
            ///
            ///     port_mapper.set(new_program).unwrap();
            ///
            ///     // Query the running program
            ///     let program_query = Mapping {
            ///         // ID, version and protocol of the program to search for
            ///         program: 10123,
            ///         version: 1,
            ///         protocol: Protocol::Tcp,
            ///         // Ignored by remote procedure
            ///         port: 0,
            ///     };
            ///
            ///     let port = port_mapper.get_port(program_query).unwrap();
            ///
            ///     assert_eq!(port, 55300);
            /// }
            use SyncClient as PortMapperSyncClient;

            use Server as PortMapperServerWrapper;
        }

        procedures {
            0 => null() -> NullResult<()>,
            1 => set(program: Mapping) -> SetResult<bool>,
            2 => unset(program: Mapping) -> UnsetResult<bool>,
            3 => get_port(program: Mapping) -> GetPortResult<u32>,
            4 => dump() -> DumpResult<Vec<Mapping>>,
            5 => call_broadcast(arguments: CallArgs)
                -> CallBroadcastResult<CallResult>,
        }
    }
}

pub type PortMapperConnect = Connect<port_mapper::AsyncClient>;
