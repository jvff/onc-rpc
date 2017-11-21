use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::service::Connect;

onc_rpc! {
    program(port_mapper::PortMapper) {
        id = 100_000;
        version = 2;

        export {
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
