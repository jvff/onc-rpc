use super::call_args::CallArgs;
use super::call_result::CallResult;
use super::mapping::Mapping;

onc_rpc! {
    program(port_mapper::PortMapper) {
        id = 100_000;
        version = 2;

        procedures {
            0 => null(),
            1 => set(program: Mapping) -> bool,
            2 => unset(program: Mapping) -> bool,
            3 => get_port(program: Mapping) -> u32,
            4 => dump() -> Vec<Mapping>,
            5 => call_broadcast(arguments: CallArgs) -> CallResult,
        }
    }
}

pub use self::port_mapper::{CallResponse, Connect, PortMapper, Program, Request,
                            Response, ServiceConfig};

pub use self::port_mapper::procedures::get_port::
    ResponseResult as GetPortResult;
