use super::super::program::PortMapperProgram;
use super::super::requests::request;
use super::super::requests::{CallArgs, CallResult};

type Program = PortMapperProgram;

onc_rpc_program_procedure!(call_broadcast(arguments: CallArgs) -> CallResult);

pub use self::call_broadcast::{Parameters, Procedure};
