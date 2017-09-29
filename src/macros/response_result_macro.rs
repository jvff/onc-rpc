#[macro_export]
macro_rules! onc_rpc_program_procedure_response_result {
    (
        @processed $procedure:ident -> $result:ty
            $( = $parameter:tt )* => $return:tt
    ) => {
        pub struct ResponseResult {
            response: CallResponse,
        }

        impl From<CallResponse> for ResponseResult {
            fn from(response: CallResponse) -> Self {
                ResponseResult { response }
            }
        }

        impl Future for ResponseResult {
            type Item = $result;
            type Error = Error;

            fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                let response = self.response.poll().chain_err(|| {
                    ErrorKind::ProcedureCallError(
                        stringify!($procedure).to_string(),
                    )
                });

                match try_ready!(response) {
                    Response::$procedure $( $parameter )* => {
                        Ok(Async::Ready($return))
                    }
                    _ => {
                        bail!(
                            ErrorKind::InvalidProcedureResponse(
                                stringify!($procedure).to_string(),
                            )
                        )
                    }
                }
            }
        }
    };

    ( $procedure:ident ) => {
        onc_rpc_program_procedure_response_result! {
            @processed
            $procedure -> () => ()
        }
    };

    ( $procedure:ident -> $result:ty ) => {
        onc_rpc_program_procedure_response_result! {
            @processed
            $procedure -> $result = (result) => result
        }
    };
}
