#[macro_export]
macro_rules! onc_rpc_program_server_response_future {
    (
        $program:ident,
        $( $procedure:ident -> $result_future:ident ),*
        $(,)*
    ) => {
        #[allow(non_camel_case_types)]
        pub enum ResponseFuture<S>
        where
            S: $program,
        {
            $( $procedure(S::$result_future), )*
        }

        impl<S> Future for ResponseFuture<S>
        where
            S: $program,
            Error: From<S::Error>,
        {
            type Item = Response;
            type Error = Error;

            fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                match *self {
                    $(
                        ResponseFuture::$procedure(ref mut future) => {
                            let result = try_ready!(future.poll());

                            Ok(Async::Ready(Response::$procedure(result)))
                        }
                    )*
                }
            }
        }
    };
}
