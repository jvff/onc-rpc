use super::call_args::CallArgs;
use super::mapping::Mapping;

pub enum Request {
    Null,
    Set(Mapping),
    Unset(Mapping),
    GetPort(Mapping),
    Dump,
    CallBroadcast(CallArgs),
}
