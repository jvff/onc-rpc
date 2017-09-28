use super::super::program::PortMapperProgram;
use super::super::requests::request;
use super::super::requests::Mapping;

type Program = PortMapperProgram;

onc_rpc_program_procedure!(dump() -> Vec<Mapping>);

pub use self::dump::Parameters;
pub use self::dump::Procedure as Dump;
