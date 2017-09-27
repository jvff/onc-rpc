use super::rpc_procedure::RpcProcedure;
use super::super::message::AuthData;

pub trait RpcCall {
    type Procedure: RpcProcedure;

    fn parameters(&self) -> <Self::Procedure as RpcProcedure>::Parameters;

    fn credentials(&self) -> AuthData {
        AuthData::default()
    }

    fn verifier(&self) -> AuthData {
        AuthData::default()
    }
}
