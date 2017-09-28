use super::super::program::PortMapperProgram;
use super::super::requests::request;
use super::super::requests::Mapping;

type Program = PortMapperProgram;

onc_rpc_program_procedure!(unset(program: Mapping) -> bool);

pub use self::unset::Parameters;
pub use self::unset::Procedure as Unset;
