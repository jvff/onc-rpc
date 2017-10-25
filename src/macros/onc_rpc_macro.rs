#[macro_export]
macro_rules! onc_rpc {
    (
        $(
            program ( $module:ident :: $name:ident ) {
                id = $id:expr ;
                version = $version:expr ;
                $( connect = $connect_alias:ident ; )*
                $( async_client = $async_client_alias:ident ; )*
                $( sync_client = $sync_client_alias:ident ; )*
                $( server = $server_alias:ident ; )*

                procedures $procedures:tt
            }
        )*
    ) => {
        $(
            onc_rpc_program!($module, $name, $id, $version, $procedures);

            pub use self::$module::$name;

            $( pub use self::$module::AsyncClient as $async_client_alias; )*
            $( pub use self::$module::SyncClient as $sync_client_alias; )*
            $( pub use self::$module::Server as $server_alias; )*

            $( pub type $connect_alias = $crate::FindPortAndConnect<$name>; )*
        )*
    };
}
