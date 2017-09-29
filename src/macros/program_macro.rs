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
                    $( -> $result_type:ty )*
                    $( => $result_future_alias:ident )*
            ),*
            $(,)*
        }
        $(,)*
    ) => {
        pub mod $module {
            use std::net::{IpAddr, SocketAddr};

            use futures::future::{Flatten, FutureResult};
            use tokio_core::net::TcpStream;
            use tokio_core::reactor::Handle;
            use tokio_proto::multiplex::{ClientFuture, ClientService};
            use tokio_service::Service;

            use $crate::{CallFuture, Error, RecordProtocol, RpcProgram,
                         RpcServiceConfig, RpcService};

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

            onc_rpc_program_connect!($name);
            onc_rpc_program_find_port_and_connect!($name, $id, $version);

            onc_rpc_program_request! {
                $(
                    $procedure_id => $procedure $parameters
                        $( -> $result_type )*
                ),*
            }

            onc_rpc_program_response! {
                $( $procedure $( -> $result_type )* ),*
            }

            pub use self::response::Response;

            onc_rpc_program_procedures! {
                $( $procedure $parameters $( -> $result_type )* ),*
            }

            use self::procedures::ProcedureMessage;

            pub struct ServiceConfig;

            impl RpcServiceConfig for ServiceConfig {
                type Request = Request;
                type Response = Response;
                type ProcedureMessage = ProcedureMessage;
            }

            pub struct $name {
                pub rpc_service: RpcService<RecordService, ServiceConfig>,
            }

            impl From<RecordService> for $name {
                fn from(record_service: RecordService) -> Self {
                    $name {
                        rpc_service: RpcService::from(record_service),
                    }
                }
            }

            impl $name {
                pub fn connect(address: IpAddr, handle: &Handle)
                    -> FindPortAndConnect
                {
                    FindPortAndConnect::new(address, handle)
                }

                pub fn connect_to_known_port(
                    address: SocketAddr,
                    handle: &Handle,
                ) -> Connect {
                    Connect::new(address, handle)
                }

                $(
                    onc_rpc_program_method!{
                        $procedure $parameters $( -> $result_type )*
                    }
                )*
            }
        }

        $(
            $(
                pub use self::$module::procedures::$procedure::ResponseResult
                    as $result_future_alias;
            )*
        )*
    };
}

#[macro_export]
macro_rules! onc_rpc_program_method {
    ( $procedure:ident () ) => {
        onc_rpc_program_method!($procedure() -> ());
    };

    ( $procedure:ident () -> $result:ty ) => {
        pub fn $procedure(&self) -> procedures::$procedure::ResponseResult {
            let request = Request::$procedure;

            self.rpc_service.call(request).into()
        }
    };

    ( $procedure:ident ( $parameter:ident : $type:ty $(,)* ) ) => {
        onc_rpc_program_method!($procedure($parameter: $type) -> ());
    };

    (
        $procedure:ident ( $parameter:ident : $type:ty $(,)* ) -> $result:ty
    ) => {
        pub fn $procedure<P>(
            &self,
            $parameter: P,
        ) -> procedures::$procedure::ResponseResult
        where
            P: Into<$type>,
        {
            let request = Request::$procedure($parameter.into());

            self.rpc_service.call(request).into()
        }
    };

    ( $procedure:ident ( $( $parameter:ident : $type:ty ),* $(,)* ) ) => {
        onc_rpc_program_method!($procedure($parameter: $type) -> ());
    };

    (
        $procedure:ident ( $( $parameter:ident : $type:ty ),* $(,)* )
            -> $result:ty
    ) => {
        pub fn $procedure(
            &self,
            $( $parameter: $type, )*
        ) -> procedures::$procedure::ResponseResult {
            let request = Request::$procedure( $( $parameter ),* );

            self.rpc_service.call(request).into()
        }
    };
}
