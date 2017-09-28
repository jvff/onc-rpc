use super::call_result::CallResult;
use super::mapping::Mapping;
use super::super::procedures;

onc_rpc_program_response! {
    null,
    set -> bool,
    unset -> bool,
    get_port -> u32,
    dump -> Vec<Mapping>,
    call_broadcast -> CallResult,
}

pub use self::response::Response as RequestResult;
