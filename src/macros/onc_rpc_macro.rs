#[macro_export]
macro_rules! onc_rpc {
    (
        $(
            program ( $module:ident :: $name:ident ) {
                id = $id:expr ;
                version = $version:expr ;
                $( connect = $connect_alias:ident ; )*

                procedures $procedures:tt
            }
        )*
    ) => {
        $(
            onc_rpc_program!($module, $name, $id, $version, $procedures);

            pub use self::$module::$name;

            $( pub use self::$module::FindPortAndConnect as $connect_alias; )*
        )*
    };
}
