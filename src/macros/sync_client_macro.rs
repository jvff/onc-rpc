#[macro_export]
macro_rules! onc_rpc_program_sync_client {
    (
        $program:ident,
        $id:expr,
        $version:expr,
        { $( $procedure:ident $parameters:tt -> $result_type:ty ),* $(,)* }
        $(,)*
    ) => {
        pub struct SyncClient {
            reactor: Core,
            async_client: AsyncClient,
        }

        impl SyncClient {
            pub fn connect(address: IpAddr) -> Result<Self> {
                let reactor = Core::new()?;
                let connect =
                    FindPortAndConnect::new(address, $id, $version, handle);

                let async_client = reactor.run(connect)?;

                Ok(SyncClient { reactor, async_client })
            }

            pub fn connect_to_known_port(address: SocketAddr) -> Result<Self> {
                let reactor = Core::new()?;
                let connect = Connect::new(address, handle);

                let async_client = reactor.run(connect)?;

                Ok(SyncClient { reactor, async_client })
            }

            $(
                onc_rpc_program_sync_client_method! {
                    $procedure $parameters -> $result_type
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! onc_rpc_program_sync_client_method {
    ( $procedure:ident () -> $result_type:ty ) => {
        fn $procedure(&self) -> Result<$result_type> {
            let operation = self.async_client.$procedure();

            self.reactor.run(operation)
        }
    };

    (
        $procedure:ident
        ( $parameter:ident : $type:ty $(,)* )
        -> $result_type:ty
    ) => {
        fn $procedure<P>(&self, $parameter: P) -> Result<$result_type>
        where
            P: Into<$type>,
        {
            let operation = self.async_client.$procedure($parameter.into());

            self.reactor.run(operation)
        }
    };

    (
        $procedure:ident
        ( $( $parameter:ident : $type:ty ),* $(,)* )
        -> $result_type:ty
    ) => {
        fn $procedure(&self, $( $parameter: $type, )*) -> Result<$result_type> {
            let operation = self.async_client.$procedure( $( $parameter ),* );

            self.reactor.run(operation)
        }
    };
}

