#[macro_export]
macro_rules! onc_rpc {
    (
        $(
            program ( $module:ident :: $name:ident ) {
                id = $id:expr ;
                version = $version:expr ;

                $( export $exports:tt )*

                procedures $procedures:tt
            }
        )*
    ) => {
        $(
            onc_rpc_program!($module, $name, $id, $version, $procedures);

            pub use self::$module::$name;

            $( onc_rpc_program_export!($module $exports); )*
        )*
    };
}
