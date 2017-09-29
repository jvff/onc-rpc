use super::port_mapper::CallResponse;
use super::program::PortMapperProgram as Program;
use super::requests::request;
use super::requests::{CallArgs, CallResult, Mapping, Response};

onc_rpc_program_procedures! {
    null(),
    set(program: Mapping) -> bool,
    unset(program: Mapping) -> bool,
    get_port(program: Mapping) -> u32,
    dump() -> Vec<Mapping>,
    call_broadcast(arguments: CallArgs) -> CallResult,
}

pub use self::procedures::ProcedureMessage;
