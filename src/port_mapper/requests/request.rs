use super::call_args::CallArgs;
use super::mapping::Mapping;
use super::request_id::RequestId;
use super::request_result::RequestResult;
use super::super::procedures::PortMapperProcedure;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Request {
    Null,
    Set(Mapping),
    Unset(Mapping),
    GetPort(Mapping),
    Dump,
    CallBroadcast(CallArgs),
}

impl PortMapperProcedure for Request {
    type ResultData = RequestResult;

    fn procedure(&self) -> u32 {
        RequestId::from(self).procedure()
    }
}
