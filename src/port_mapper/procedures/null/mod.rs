use super::super::program::PortMapperProgram;
use super::super::requests::request;

type Program = PortMapperProgram;

onc_rpc_program_procedure!(null());

pub use self::null::Parameters;
pub use self::null::Procedure as Null;
