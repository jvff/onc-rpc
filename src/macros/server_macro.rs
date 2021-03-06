#[macro_export]
macro_rules! onc_rpc_program_server {
    (
        @ready
        $program:ident,
        $( #[$attr:meta] )* Server,
        { $( $procedure:ident $parameters:tt -> $result_future:ident ),* $(,)* }
        $(,)*
    ) => {
        pub mod server {
            use std::io;
            use std::net::SocketAddr;

            use futures::{Async, Future, Poll};
            use tokio_service::{NewService, Service};

            use super::{Error, Request, Response, ServiceConfig};
            use super::$program;

            use $crate::RpcServer;

            onc_rpc_program_server_response_future! {
                $program,
                $( $procedure -> $result_future ),*
            }

            onc_rpc_program_server_service! {
                $program,
                { $( $procedure $parameters -> $result_future ),* }
            }

            $( #[$attr] )*
            pub struct Server {
                server: RpcServer<ServiceConfig>,
            }

            impl Server {
                /// Create a new server to listen on a given address.
                pub fn new(address: SocketAddr) -> Self {
                    Server {
                        server: RpcServer::new(address),
                    }
                }

                /// Start listening and accepting remote procedure calls.
                ///
                /// Calls are delegated to the specified program. For every
                /// call, the program instance is cloned in order to process the
                /// request asynchronously. Care must be taken to make sure the
                /// program's internal state is shared when cloned.
                pub fn serve<P>(&self, program: P)
                where
                    P: 'static + Clone + $program + Send + Sync,
                    Error: From<P::Error>,
                {
                    self.server.serve_rpc_service(ServerService::from(program));
                }
            }
        }

        pub use self::server::Server;
    };

    (
        $program:ident,
        {},
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_server!(@ready $program, Server, $procedures);
    };

    (
        $program:ident,
        {
            $( #[$server_attr:meta] )* use Server as $server:ident;
            $( $( #[$attr:meta] )* use $type:ident as $alias:ident; )*
        },
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_server! {
            @ready
            $program,
            $( #[$server_attr] )* Server,
            $procedures,
        }
    };

    (
        $program:ident,
        {
            $( #[$ignored_attr:meta] )*
            use $ignored_type:ident as $ignored_alias:ident;

            $( $( #[$attr:meta] )* use $type:ident as $alias:ident; )*
        },
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_server! {
            $program,
            {
                $( $( #[$attr] )* use $type as $alias; )*
            },
            $procedures,
        }
    };
}
