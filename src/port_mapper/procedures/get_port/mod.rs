use super::super::program::PortMapperProgram;
use super::super::requests::request;
use super::super::requests::Mapping;

type Program = PortMapperProgram;

onc_rpc_program_procedure!(get_port(program: Mapping) -> u32);

pub use self::get_port::Parameters;
pub use self::get_port::Procedure as GetPort;
