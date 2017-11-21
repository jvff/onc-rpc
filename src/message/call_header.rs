use super::auth_data::AuthData;
use super::super::rpc::{RpcCall, RpcProcedure, RpcProgram};

/// Message header for a remote procedure call.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallHeader {
    rpc_version: u32,
    program: u32,
    version: u32,
    procedure: u32,
    credentials: AuthData,
    verifier: AuthData,
}

impl<'c, C> From<&'c C> for CallHeader
where
    C: RpcCall,
{
    fn from(rpc_call: &'c C) -> Self {
        let program =
            <<C::Procedure as RpcProcedure>::Program as RpcProgram>::program();

        let version =
            <<C::Procedure as RpcProcedure>::Program as RpcProgram>::version();

        let procedure = <C::Procedure as RpcProcedure>::procedure();

        CallHeader {
            rpc_version: 2,
            program,
            version,
            procedure,
            credentials: rpc_call.credentials(),
            verifier: rpc_call.verifier(),
        }
    }
}

impl CallHeader {
    /// Get the ID of the procedure to call.
    pub fn procedure(&self) -> u32 {
        self.procedure
    }
}
