#[macro_export]
macro_rules! onc_rpc_program_client {
    (
        $program:ident,
        $id:expr,
        $version:expr,
        {
            $(
                $procedure:ident
                $parameters:tt
                -> $result_future:ident < $result_type:ty >
            ),*
            $(,)*
        }
        $(,)*
    ) => {
        onc_rpc_program_async_client! {
            $program,
            $id,
            $version,
            {
                $( $procedure $parameters -> $result_future ),*
            }
        }
    };
}
