#[macro_export]
macro_rules! onc_rpc_program_client {
    (
        $program:ident,
        $id:expr,
        $version:expr,
        { $( $procedure:ident $parameters:tt -> $result_future:ident ),* $(,)* }
        $(,)*
    ) => {
        pub struct Client {
            pub rpc_service: RpcClientService<RecordService, ServiceConfig>,
        }

        impl From<RecordService> for Client {
            fn from(record_service: RecordService) -> Self {
                Client {
                    rpc_service: RpcClientService::from(record_service),
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

        impl $program for Client {
            type Error = Error;

            $(
                type $result_future = procedures::$procedure::ResponseResult;
            )*

            $( onc_rpc_program_client_method!($procedure $parameters); )*
        }
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

