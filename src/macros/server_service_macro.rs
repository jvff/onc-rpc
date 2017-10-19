#[macro_export]
macro_rules! onc_rpc_program_server_service {
    (
        @end_marker
        $program:ident,
        $(
            $procedure:ident
            $( ($parameter:ident) )*
            $( { $( $field:ident ),* } )*
            => $parameters:tt
            -> $result_future:ident
        ),*
        $(,)*
    ) => {
        pub struct ServerService<P>
        where
            P: $program,
        {
            program: P,
        }

        impl<P> From<P> for ServerService<P>
        where
            P: $program,
        {
            fn from(program: P) -> Self {
                ServerService { program }
            }
        }

        impl<P> Service for ServerService<P>
        where
            P: $program,
            Error: From<P::Error>,
        {
            type Request = Request;
            type Response = Response;
            type Error = Error;
            type Future = ResponseFuture<P>;

            fn call(&self, request: Self::Request) -> Self::Future {
                match request {
                    $(
                        Request::$procedure
                            $( ($parameter) )*
                            $( { $( $field ),* } )*
                        => {
                            let response = self.program.$procedure $parameters;

                            ResponseFuture::$procedure(response)
                        }
                    )*
                }
            }
        }

        impl<P> NewService for ServerService<P>
        where
            P: Clone + $program,
            Error: From<P::Error>,
        {
            type Request = Request;
            type Response = Response;
            type Error = Error;
            type Instance = ServerService<P>;

            fn new_service(&self) -> io::Result<Self::Instance> {
                Ok(self.program.clone().into())
            }
        }
    };

    (
        $program:ident,
        { $( $procedure:ident $parameters:tt -> $result_future:ident ),* $(,)* }
        $(,)*
    ) => {
        onc_rpc_program_server_service! {
            $( $procedure $parameters -> $result_future ),*
            @end_marker
            $program,
        }
    };

    (
        $procedure:ident () -> $result_future:ident
        $(
            ,
            $next_procedure:ident
            $next_parameters:tt
            -> $next_result_future:ident
        )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_server_service! {
            $( $next_procedure $next_parameters -> $next_result_future ),*
            @end_marker
            $( $ready )*
            $procedure => () -> $result_future,
        }
    };

    (
        $procedure:ident ($name:ident : $type:ty) -> $result_future:ident
        $(
            ,
            $next_procedure:ident
            $next_parameters:tt
            -> $next_result_future:ident
        )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_server_service! {
            $( $next_procedure $next_parameters -> $next_result_future ),*
            @end_marker
            $( $ready )*
            $procedure($name) => ($name) -> $result_future,
        }
    };

    (
        $procedure:ident ( $( $name:ident : $type:ty ),* $(,)* )
        -> $result_future:ident
        $(
            ,
            $next_procedure:ident
            $next_parameters:tt
            -> $next_result_future:ident
        )*
        @end_marker
        $( $ready:tt )*
    ) => {
        onc_rpc_program_server_service! {
            $( $next_procedure $next_parameters -> $next_result_future ),*
            @end_marker
            $( $ready )*
            $procedure { $( $name ),* } => ( $( $name ),* ) -> $result_future,
        }
    };
}
