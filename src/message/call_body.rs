use super::call_header::CallHeader;
use super::super::rpc::{RpcCall, RpcProcedure};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallBody<T> {
    header: CallHeader,
    parameters: T,
}

impl<C, T> From<C> for CallBody<T>
where
    C: RpcCall,
    C::Procedure: RpcProcedure<Parameters = T>,
{
    fn from(rpc_call: C) -> Self {
        let header = CallHeader::from(&rpc_call);

        CallBody {
            header,
            parameters: rpc_call.parameters(),
        }
    }
}

impl<T> CallBody<T> {
    pub fn new(header: CallHeader, parameters: T) -> Self {
        CallBody { header, parameters }
    }
}
