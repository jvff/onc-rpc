macro_rules! onc_rpc {
    (
        $(
            program ( $module:ident :: $name:ident ) {
                id = $id:expr ;
                version = $version:expr ;

                procedures $procedures:tt
            }
        )*
    ) => {
        $(
            onc_rpc_program!($module, $name, $id, $version, $procedures);
        )*
    };
}
