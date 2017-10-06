#[macro_export]
macro_rules! onc_rpc_program_request {
    ( $( $id:expr => $procedure:ident $parameters:tt ),* $(,)* ) => {
        mod request {
            use $crate::RpcRequest;

            use super::*;

            onc_rpc_program_request_id! {
                $( $id => $procedure $parameters, )*
            }

            onc_rpc_program_request_enum! {
                $( $id => $procedure $parameters, )*
            }

            impl RpcRequest for Request {
                type ResponseHint = RequestId;

                fn response_hint(&self) -> RequestId {
                    RequestId::from(self)
                }
            }
        }

        pub use self::request::{Request, RequestId};
    };
}

#[macro_export]
macro_rules! onc_rpc_program_request_enum {
    (
        ; end_marker ;
        $(
            $request:ident
            $( ( $parameter:ty ) )*
            $( { $( $name:ident: $type:ty, )* } )*
            ,
        )*
    ) => {
        #[allow(non_camel_case_types)]
        pub enum Request {
            $( $request $( ($parameter) )* $( { $( $name: $type, )* } )*, )*
        }
    };

    (
        $(
            $id:expr => $procedure:ident ( $( $name:ident: $type:ty ),* $(,)* )
        ),*
        $(,)*
    ) => {
        onc_rpc_program_request_enum! {
            $( $id => $procedure ( $( $name: $type ),* ) ),*
            ; end_marker ;
        }
    };

    (
        $id:expr => $procedure:ident ()
        $( , $next_id:expr => $next_procedure:ident $next_parameters:tt )*
        ; end_marker ;
        $(
            $request:ident
            $( ( $parameter:ty ) )*
            $( { $( $request_name:ident: $request_type:ty, )* } )*
            ,
        )*
    ) => {
        onc_rpc_program_request_enum! {
            $( $next_id => $next_procedure $next_parameters ),*
            ; end_marker ;
            $(
                $request
                $( ($parameter) )*
                $( { $( $request_name: $request_type, )* } )*
                ,
            )*
            $procedure,
        }
    };

    (
        $id:expr => $procedure:ident ( $name:ident : $type:ty )
        $( , $next_id:expr => $next_procedure:ident $next_parameters:tt )*
        ; end_marker ;
        $(
            $request:ident
            $( ( $parameter:ty ) )*
            $( { $( $request_name:ident: $request_type:ty, )* } )*
            ,
        )*
    ) => {
        onc_rpc_program_request_enum! {
            $( $next_id => $next_procedure $next_parameters ),*
            ; end_marker ;
            $(
                $request
                $( ($parameter) )*
                $( { $( $request_name: $request_type, )* } )*
                ,
            )*
            $procedure($type),
        }
    };

    (
        $id:expr => $procedure:ident ( $( $name:ident : $type:ty ),* )
        $( , $next_id:expr => $next_procedure:ident $next_parameters:tt )*
        ; end_marker ;
        $(
            $request:ident
            $( ( $parameter:ty ) )*
            $( { $( $request_name:ident: $request_type:ty, )* } )*
            ,
        )*
    ) => {
        onc_rpc_program_request_enum! {
            $( $next_id => $next_procedure $next_parameters ),*
            ; end_marker ;
            $(
                $request
                $( ($parameter) )*
                $( { $( $request_name: $request_type, )* } )*
                ,
            )*
            $procedure {
                $( $name: $type, )*
            },
        }
    };
}
