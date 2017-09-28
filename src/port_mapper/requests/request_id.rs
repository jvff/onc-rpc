use super::request::Request;

onc_rpc_program_request_id! {
    0 => null(),
    1 => set(program: Mapping) -> bool,
    2 => unset(program: Mapping) -> bool,
    3 => get_port(program: Mapping) -> u32,
    4 => dump() -> Vec<Mapping>,
    5 => call_broadcast(arguments: CallArgs) -> CallResult,
}
