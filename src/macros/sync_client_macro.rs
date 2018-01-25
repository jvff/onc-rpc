#[macro_export]
macro_rules! onc_rpc_program_sync_client {
    (
        @ready
        $program:ident,
        $id:expr,
        $version:expr,
        $( #[$attr:meta] )* SyncClient,
        {
            $( $( #[$procedure_attribute:meta] )*
            $procedure:ident $parameters:tt -> $result_type:ty ),* $(,)*
        }
        $(,)*
    ) => {
        mod sync_client {
            use tokio_core::reactor::Core;

            use super::*;

            use $crate::Result;

            $( #[$attr] )*
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
                        $( #[$procedure_attribute] )*
                        $procedure $parameters -> $result_type
                    }
                )*
            }
        }

        pub use self::sync_client::SyncClient;
    };

    (
        $program:ident,
        $id:expr,
        $version:expr,
        {},
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_sync_client! {
            @ready
            $program,
            $id,
            $version,
            SyncClient,
            $procedures,
        }
    };

    (
        $program:ident,
        $id:expr,
        $version:expr,
        {
            $( #[$sync_attr:meta] )* use SyncClient as $sync_client:ident;
            $( $( #[$attr:meta] )* use $type:ident as $alias:ident; )*
        },
        $procedures:tt
        $(,)*
    ) => {
        onc_rpc_program_sync_client! {
            @ready
            $program,
            $id,
            $version,
            $( #[$sync_attr] )* SyncClient,
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
        onc_rpc_program_sync_client! {
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
macro_rules! onc_rpc_program_sync_client_method {
    ( $( #[$attribute:meta] )* $procedure:ident () -> $result_type:ty ) => {
        $( #[$attribute] )*
        pub fn $procedure(&mut self) -> Result<$result_type> {
            let operation = self.async_client.$procedure();

            self.reactor.run(operation)
        }
    };

    (
        $( #[$attribute:meta] )*
        $procedure:ident
        ( $parameter:ident : $type:ty $(,)* )
        -> $result_type:ty
    ) => {
        $( #[$attribute] )*
        pub fn $procedure<P>(&mut self, $parameter: P) -> Result<$result_type>
        where
            P: Into<$type>,
        {
            let operation = self.async_client.$procedure($parameter.into());

            self.reactor.run(operation)
        }
    };

    (
        $( #[$attribute:meta] )*
        $procedure:ident
        ( $( $parameter:ident : $type:ty ),* $(,)* )
        -> $result_type:ty
    ) => {
        $( #[$attribute] )*
        pub fn $procedure(
            &mut self, $( $parameter: $type, )*
        ) -> Result<$result_type> {
            let operation = self.async_client.$procedure( $( $parameter ),* );

            self.reactor.run(operation)
        }
    };
}
