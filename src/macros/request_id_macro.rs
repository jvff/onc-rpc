#[macro_export]
macro_rules! onc_rpc_program_request_id {
    (
        $(
            $id:expr => $procedure:ident $parameters:tt
               $( -> $return_type:ty )*
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

macro_rules! onc_rpc_program_request_id_enum {
    (
        $(
            $id:expr => $procedure:ident $parameters:tt
               $( -> $return_type:ty )*
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
        $(
            $id:expr => $procedure:ident $parameters:tt $( -> $return_type:ty )*
        ),*
        $(,)*
    ) => {
        onc_rpc_program_request_id_from! {
            $( $id => $procedure $parameters $( -> $return_type )* ),*
            ; end_marker ;
        }
    };

    (
        $id:expr => $procedure:ident ( $(,)* ) $( -> $return_type:ty )*
        $(
            , $next_id:expr => $next_procedure:ident $next_parameters:tt
                $( -> $next_return_type:ty )*
        )*
        ; end_marker ;
        $( $request:pat => $resolved_id:expr, )*
    ) => {
        onc_rpc_program_request_id_from! {
            $(
                $next_id => $next_procedure $next_parameters
                    $( -> $next_return_type )*
            ),*
            ; end_marker ;
            $( $request => $resolved_id, )*
            Request::$procedure => RequestId::$procedure,
        }
    };

    (
        $id:expr => $procedure:ident $parameters:tt $( -> $return_type:ty )*
        $(
            , $next_id:expr => $next_procedure:ident $next_parameters:tt
                $( -> $next_return_type:ty )*
        )*
        ; end_marker ;
        $( $request:pat => $resolved_id:expr, )*
    ) => {
        onc_rpc_program_request_id_from! {
            $(
                $next_id => $next_procedure $next_parameters
                    $( -> $next_return_type )*
            ),*
            ; end_marker ;
            $( $request => $resolved_id, )*
            Request::$procedure(_) => RequestId::$procedure,
        }
    };
}
