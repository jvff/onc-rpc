use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::procedures::ProcedureMessage;
use super::super::super::errors::Result;

pub enum RequestResult {
    Null,
    Set(bool),
    Unset(bool),
    GetPort(u32),
    Dump(Vec<Mapping>),
    CallBroadcast(CallResult),
}

impl RequestResult {
    pub fn try_from(reply: ProcedureMessage) -> Result<Self> {
        match reply {
            ProcedureMessage::Null(_) => Ok(RequestResult::Null),
            ProcedureMessage::Set(message) => {
                Ok(RequestResult::Set(message.into_reply()?))
            }
            ProcedureMessage::Unset(message) => {
                Ok(RequestResult::Unset(message.into_reply()?))
            }
            ProcedureMessage::GetPort(message) => {
                Ok(RequestResult::GetPort(message.into_reply()?))
            }
            ProcedureMessage::Dump(message) => {
                Ok(RequestResult::Dump(message.into_reply()?))
            }
            ProcedureMessage::CallBroadcast(message) => {
                Ok(RequestResult::CallBroadcast(message.into_reply()?))
            }
        }
    }
}
