use super::requests::{CallArgs, CallResult, Mapping};

onc_rpc_program! {
    port_mapper,
    PortMapper,
    100_000,
    2,
    {
        0 => null(),
        1 => set(program: Mapping) -> bool,
        2 => unset(program: Mapping) -> bool,
        3 => get_port(program: Mapping) -> u32,
        4 => dump() -> Vec<Mapping>,
        5 => call_broadcast(arguments: CallArgs) -> CallResult,
    }
}

pub use self::port_mapper::{CallResponse, Connect, PortMapper, Program, Request,
                            Response, ServiceConfig};
