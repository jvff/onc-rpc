use super::call_body::CallBody;
use super::reply_body::ReplyBody;
use super::super::errors::{ErrorKind, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RpcBody<C, R> {
    Call(CallBody<C>),
    Reply(ReplyBody<R>),
}

impl<C, R> RpcBody<C, R> {
    pub fn into_reply(self) -> Result<R> {
        match self {
            RpcBody::Call(_) => bail!(ErrorKind::CantConvertCallToResult),
            RpcBody::Reply(body) => body.into(),
        }
    }
}
