#[macro_export]
macro_rules! onc_rpc_program_export {
    (
        $module:ident {
            $( $type:ident as $alias:ident; )*
        }
    ) => {
        $( pub use self::$module::$type as $alias; )*
    };
}
