use super::call_result::CallResult;
use super::mapping::Mapping;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum RequestResult {
    Null,
    Set(bool),
    Unset(bool),
    GetPort(u32),
    Dump(Vec<Mapping>),
    CallBroadcast(CallResult),
}
