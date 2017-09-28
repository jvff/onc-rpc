use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::procedures::ProcedureMessage;
use super::super::super::errors::{Error, Result};
use super::super::super::service::TryFrom;

pub enum RequestResult {
    Null,
    Set(bool),
    Unset(bool),
    GetPort(u32),
    Dump(Vec<Mapping>),
    CallBroadcast(CallResult),
}

impl TryFrom<ProcedureMessage> for RequestResult {
    type Error = Error;

    fn try_from(reply: ProcedureMessage) -> Result<Self> {
        match reply {
            ProcedureMessage::null(_) => Ok(RequestResult::Null),
            ProcedureMessage::set(message) => {
                Ok(RequestResult::Set(message.into_reply()?))
            }
            ProcedureMessage::unset(message) => {
                Ok(RequestResult::Unset(message.into_reply()?))
            }
            ProcedureMessage::get_port(message) => {
                Ok(RequestResult::GetPort(message.into_reply()?))
            }
            ProcedureMessage::dump(message) => {
                Ok(RequestResult::Dump(message.into_reply()?))
            }
            ProcedureMessage::call_broadcast(message) => {
                Ok(RequestResult::CallBroadcast(message.into_reply()?))
            }
        }
    }
}
