#[macro_export]
macro_rules! onc_rpc_program_async_client {
    (
        @ready
        $program:ident,
        $id:expr,
        $version:expr,
        $( #[$attr:meta] )* AsyncClient,
        { $( $procedure:ident $parameters:tt -> $result_future:ident ),* $(,)* }
        $(,)*
    ) => {
        $( #[$attr] )*
        pub struct AsyncClient {
            pub rpc_service: RpcClientService<RecordService, ServiceConfig>,
        }

        impl From<RecordService> for AsyncClient {
            fn from(record_service: RecordService) -> Self {
                AsyncClient {
                    rpc_service: RpcClientService::from(record_service),
                }
            }
        }

        impl AsyncClient {
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

        impl $program for AsyncClient {
            type Error = Error;

            $(
                type $result_future = procedures::$procedure::ResponseResult;
            )*

            $( onc_rpc_program_async_client_method!($procedure $parameters); )*
        }
    };

    (
        $program:ident,
        $id:expr,
        $version:expr,
        {},
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_async_client! {
            @ready
            $program,
            $id,
            $version,
            AsyncClient,
            $procedures,
        }
    };

    (
        $program:ident,
        $id:expr,
        $version:expr,
        {
            $( #[$async_attr:meta] )* use AsyncClient as $async_client:ident;
            $( $( #[$attr:meta] )* use $type:ident as $alias:ident; )*
        },
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_async_client! {
            @ready
            $program,
            $id,
            $version,
            $( #[$async_attr] )* AsyncClient,
            $procedures,
        }
    };

    (
        $program:ident,
        $id:expr,
        $version:expr,
        {
            $( #[$ignored_attr:meta] )*
            use $ignored_type:ident as $ignored_alias:ident;

            $( $( #[$attr:meta] )* use $type:ident as $alias:ident; )*
        },
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_async_client! {
            $program,
            $id,
            $version,
            {
                $( $( #[$attr] )* use $type as $alias; )*
            },
            $procedures,
        }
    };
}

#[macro_export]
macro_rules! onc_rpc_program_async_client_method {
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

