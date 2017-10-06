#[macro_export]
macro_rules! onc_rpc_program_request_id {
    (
        $(
            $id:expr => $procedure:ident $parameters:tt
        ),*
        $(,)*
    ) => {
        onc_rpc_program_request_id_enum! {
            $( $id => $procedure $parameters, )*
        }

        onc_rpc_program_request_id_from! {
            $( $id => $procedure $parameters, )*
        }
    }
}

#[macro_export]
macro_rules! onc_rpc_program_request_id_enum {
    (
        $(
            $id:expr => $procedure:ident $parameters:tt
        ),*
        $(,)*
    ) => {
        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub enum RequestId {
            $( $procedure ),*
        }

        impl RequestId {
            pub fn procedure(&self) -> u32 {
                match *self {
                    $( RequestId::$procedure => $id, )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! onc_rpc_program_request_id_from {
    ( ; end_marker ; $( $request:pat => $id:expr ),* $(,)* ) => {
        impl<'r> From<&'r Request> for RequestId {
            fn from(request: &'r Request) -> Self {
                match *request {
                    $( $request => $id, )*
                }
            }
        }
    };

    (
        $( $id:expr => $procedure:ident $parameters:tt ),*
        $(,)*
    ) => {
        onc_rpc_program_request_id_from! {
            $( $id => $procedure $parameters ),*
            ; end_marker ;
        }
    };

    (
        $id:expr => $procedure:ident ( $(,)* )
        $( , $next_id:expr => $next_procedure:ident $next_parameters:tt )*
        ; end_marker ;
        $( $request:pat => $resolved_id:expr, )*
    ) => {
        onc_rpc_program_request_id_from! {
            $( $next_id => $next_procedure $next_parameters ),*
            ; end_marker ;
            $( $request => $resolved_id, )*
            Request::$procedure => RequestId::$procedure,
        }
    };

    (
        $id:expr => $procedure:ident $parameters:tt
        $( , $next_id:expr => $next_procedure:ident $next_parameters:tt )*
        ; end_marker ;
        $( $request:pat => $resolved_id:expr, )*
    ) => {
        onc_rpc_program_request_id_from! {
            $( $next_id => $next_procedure $next_parameters ),*
            ; end_marker ;
            $( $request => $resolved_id, )*
            Request::$procedure(_) => RequestId::$procedure,
        }
    };
}
