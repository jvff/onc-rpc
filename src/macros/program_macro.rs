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
                         RecordProtocol, RpcProgram, RpcServiceConfig,
                         RpcService};

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

            pub struct Client {
                pub rpc_service: RpcService<RecordService, ServiceConfig>,
            }

            impl From<RecordService> for Client {
                fn from(record_service: RecordService) -> Self {
                    Client {
                        rpc_service: RpcService::from(record_service),
                    }
                }
            }

            impl Client {
                pub fn connect(address: IpAddr, handle: &Handle)
                    -> FindPortAndConnect<Self>
                {
                    FindPortAndConnect::new(address, $id, $version, handle)
                }

                pub fn connect_to_known_port(
                    address: SocketAddr,
                    handle: &Handle,
                ) -> Connect<Self> {
                    Connect::new(address, handle)
                }
            }

            impl $name for Client {
                type Error = Error;

                $(
                    type $result_future =
                        procedures::$procedure::ResponseResult;
                )*

                $( onc_rpc_program_client_method!($procedure $parameters); )*
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

#[macro_export]
macro_rules! onc_rpc_program_client_method {
    ( $procedure:ident () ) => {
        fn $procedure(&self) -> procedures::$procedure::ResponseResult {
            let request = Request::$procedure;

            self.rpc_service.call(request).into()
        }
    };

    ( $procedure:ident ( $parameter:ident : $type:ty $(,)* ) ) => {
        fn $procedure<P>(
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
        fn $procedure(
            &self,
            $( $parameter: $type, )*
        ) -> procedures::$procedure::ResponseResult {
            let request = Request::$procedure( $( $parameter ),* );

            self.rpc_service.call(request).into()
        }
    };
}
