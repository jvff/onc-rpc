#[macro_export]
macro_rules! onc_rpc_program_server {
    (
        $program:ident,
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

            pub struct Server {
                server: RpcServer<ServiceConfig>,
            }

            impl Server {
                pub fn new(address: SocketAddr) -> Self {
                    Server {
                        server: RpcServer::new(address),
                    }
                }

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
}
