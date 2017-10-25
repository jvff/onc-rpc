#[macro_export]
macro_rules! onc_rpc_program {
    (
        $module:ident,
        $name:ident,
        $id:expr,
        $version:expr,
        {
            $(
                $procedure_id:expr => $procedure:ident $parameters:tt
                    -> $result_future:ident < $result_type:ty >
            ),*
            $(,)*
        }
        $(,)*
    ) => {
        pub mod $module {
            use std::net::{IpAddr, SocketAddr};

            use futures::future::{Flatten, Future, FutureResult};
            use tokio_core::net::TcpStream;
            use tokio_core::reactor::Handle;
            use tokio_proto::multiplex::{ClientFuture, ClientService};
            use tokio_service::Service;

            use $crate::{CallFuture, Connect, Error, FindPortAndConnect,
                         RecordProtocol, RpcClientService, RpcProgram,
                         RpcServiceConfig};

            use super::*;

            type RecordFuture = ClientFuture<TcpStream, RecordProtocol>;
            type RecordService = ClientService<TcpStream, RecordProtocol>;

            pub type CallResponse =
                Flatten<
                    FutureResult<
                        CallFuture<RecordFuture, ServiceConfig>,
                        Error,
                    >,
                >;

            pub struct Program;

            impl RpcProgram for Program {
                fn program() -> u32 {
                    $id
                }

                fn version() -> u32 {
                    $version
                }
            }

            onc_rpc_program_request! {
                $( $procedure_id => $procedure $parameters ),*
            }

            onc_rpc_program_response! {
                $( $procedure -> $result_type ),*
            }

            pub use self::response::Response;

            onc_rpc_program_procedures! {
                $( $procedure $parameters -> $result_type ),*
            }

            use self::procedures::ProcedureMessage;

            pub struct ServiceConfig;

            impl RpcServiceConfig for ServiceConfig {
                type Request = Request;
                type Response = Response;
                type ProcedureMessage = ProcedureMessage;
            }

            pub trait $name {
                type Error;

                $(
                    type $result_future:
                        Future<Item = $result_type, Error = Self::Error>;
                )*

                $(
                    onc_rpc_program_trait_method! {
                        $procedure $parameters -> $result_future
                    }
                )*
            }

            onc_rpc_program_client! {
                $name,
                $id,
                $version,
                {
                    $(
                        $procedure
                        $parameters
                        -> $result_future < $result_type >
                    ),*
                }
            }

            onc_rpc_program_server! {
                $name,
                { $( $procedure $parameters -> $result_future ),* }
            }
        }

        $(
            pub use self::$module::procedures::$procedure::ResponseResult
                as $result_future;
        )*
    };
}

#[macro_export]
macro_rules! onc_rpc_program_trait_method {
    ( $procedure:ident () -> $result_future:ident ) => {
        fn $procedure(&self) -> Self::$result_future;
    };

    (
        $procedure:ident ( $parameter:ident : $type:ty $(,)* )
            -> $result_future:ident
    ) => {
        fn $procedure<P>(&self, $parameter: P) -> Self::$result_future
        where
            P: Into<$type>;
    };

    (
        $procedure:ident ( $( $parameter:ident : $type:ty ),* $(,)* )
            -> $result_future:ident
    ) => {
        fn $procedure(&self, $( $parameter: $type, )*) -> Self::$result_future;
    };
}
