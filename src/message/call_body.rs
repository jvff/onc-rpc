use super::auth_data::AuthData;
use super::super::rpc::{RpcCall, RpcProcedure, RpcProgram};

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

impl<C, T> From<C> for CallBody<T>
where
    C: RpcCall,
    C::Procedure: RpcProcedure<Parameters = T>,
{
    fn from(rpc_call: C) -> Self {
        let program =
            <<C::Procedure as RpcProcedure>::Program as RpcProgram>::program();

        let version =
            <<C::Procedure as RpcProcedure>::Program as RpcProgram>::version();

        let procedure = <C::Procedure as RpcProcedure>::procedure();

        CallBody {
            rpc_version: 2,
            program,
            version,
            procedure,
            credentials: rpc_call.credentials(),
            verifier: rpc_call.verifier(),
            parameters: rpc_call.parameters(),
        }
    }
}
