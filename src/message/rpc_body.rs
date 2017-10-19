use super::call_body::CallBody;
use super::reply_body::ReplyBody;
use super::super::errors::{ErrorKind, Result};
use super::super::rpc::{RpcCall, RpcProcedure};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RpcBody<C, R> {
    Call(CallBody<C>),
    Reply(ReplyBody<R>),
}

impl<C, P, R> From<C> for RpcBody<P, R>
where
    C: RpcCall,
    C::Procedure: RpcProcedure<Parameters = P, ResultData = R>,
{
    fn from(rpc_call: C) -> Self {
        RpcBody::Call(rpc_call.into())
    }
}

impl<C, R> RpcBody<C, R> {
    pub fn into_parameters(self) -> Result<C> {
        match self {
            RpcBody::Call(body) => Ok(body.into_parameters()),
            RpcBody::Reply(_) => bail!(ErrorKind::CantConvertResultToCall),
        }
    }

    pub fn into_reply(self) -> Result<R> {
        match self {
            RpcBody::Call(_) => bail!(ErrorKind::CantConvertCallToResult),
            RpcBody::Reply(body) => body.into(),
        }
    }
}
