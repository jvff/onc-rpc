use super::auth_data::AuthData;
use super::super::rpc::RpcCall;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallBody<T> {
    rpc_version: u32,
    program: u32,
    version: u32,
    procedure: u32,
    credentials: AuthData,
    verifier: AuthData,
    parameters: T,
}

impl<T, C> From<C> for CallBody<T>
where
    C: RpcCall<Parameters = T>,
{
    fn from(rpc_call: C) -> Self {
        CallBody {
            rpc_version: 2,
            program: rpc_call.program(),
            version: rpc_call.version(),
            procedure: rpc_call.procedure(),
            credentials: rpc_call.credentials(),
            verifier: rpc_call.verifier(),
            parameters: rpc_call.parameters(),
        }
    }
}
