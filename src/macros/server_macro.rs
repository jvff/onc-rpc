#[macro_export]
macro_rules! onc_rpc_program_server {
    (
        $program:ident,
        { $( $procedure:ident $parameters:tt -> $result_future:ident ),* $(,)* }
        $(,)*
    ) => {
        pub mod server {
            use std::io;

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

            pub type Server = RpcServer<ServiceConfig>;

            impl Server {
                pub fn serve<P>(&self, program: P)
                where
                    P: 'static + Clone + $program + Send + Sync,
                    Error: From<P::Error>,
                {
                    self.serve_rpc_service(ServerService::from(program));
                }
            }
        }
    };
}
