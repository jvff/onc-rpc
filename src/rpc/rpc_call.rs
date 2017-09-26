use super::rpc_procedure::RpcProcedure;
use super::super::message::AuthData;

pub trait RpcCall: RpcProcedure {
    fn parameters(&self) -> Self::Parameters;

    fn credentials(&self) -> AuthData {
        AuthData::default()
    }

    fn verifier(&self) -> AuthData {
        AuthData::default()
    }
}
