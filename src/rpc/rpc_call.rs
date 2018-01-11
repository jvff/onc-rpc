use super::rpc_procedure::RpcProcedure;
use super::super::message::AuthData;

/// Representation of a remote procedure call.
///
/// Any type that implements this trait provides enough information to create a
/// remote procedure call request message.
pub trait RpcCall {
    /// The requested procedure.
    type Procedure: RpcProcedure;

    /// The parameters for the call.
    fn parameters(&self) -> <Self::Procedure as RpcProcedure>::Parameters;

    /// The credentials to perform the call.
    fn credentials(&self) -> AuthData {
        AuthData::default()
    }

    /// The verifier to use for the credentials.
    fn verifier(&self) -> AuthData {
        AuthData::default()
    }
}
