use super::call_args::CallArgs;
use super::mapping::Mapping;

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
