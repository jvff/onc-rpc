#[macro_export]
macro_rules! onc_rpc_program_sync_client {
    (
        $program:ident,
        $id:expr,
        $version:expr,
        { $( $procedure:ident $parameters:tt -> $result_type:ty ),* $(,)* }
        $(,)*
    ) => {
        mod sync_client {
            use tokio_core::reactor::Core;

            use super::*;

            use $crate::Result;

            pub struct SyncClient {
                reactor: Core,
                async_client: AsyncClient,
            }

            impl SyncClient {
                pub fn connect(address: IpAddr) -> Result<Self> {
                    let mut reactor = Core::new()?;
                    let handle = reactor.handle();
                    let connect = AsyncClient::connect(address, &handle);

                    let async_client = reactor.run(connect)?;

                    Ok(SyncClient { reactor, async_client })
                }

                pub fn connect_to_known_port(
                    address: SocketAddr,
                ) -> Result<Self> {
                    let mut reactor = Core::new()?;
                    let handle = reactor.handle();
                    let connect =
                        AsyncClient::connect_to_known_port(address, &handle);

                    let async_client = reactor.run(connect)?;

                    Ok(SyncClient { reactor, async_client })
                }

                $(
                    onc_rpc_program_sync_client_method! {
                        $procedure $parameters -> $result_type
                    }
                )*
            }
        }

        pub use self::sync_client::SyncClient;
    };
}

#[macro_export]
macro_rules! onc_rpc_program_sync_client_method {
    ( $procedure:ident () -> $result_type:ty ) => {
        pub fn $procedure(&mut self) -> Result<$result_type> {
            let operation = self.async_client.$procedure();

            self.reactor.run(operation)
        }
    };

    (
        $procedure:ident
        ( $parameter:ident : $type:ty $(,)* )
        -> $result_type:ty
    ) => {
        pub fn $procedure<P>(&mut self, $parameter: P) -> Result<$result_type>
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
        pub fn $procedure(
            &mut self, $( $parameter: $type, )*
        ) -> Result<$result_type> {
            let operation = self.async_client.$procedure( $( $parameter ),* );

            self.reactor.run(operation)
        }
    };
}

