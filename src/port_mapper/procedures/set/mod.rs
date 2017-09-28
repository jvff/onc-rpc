use super::super::program::PortMapperProgram;
use super::super::requests::request;
use super::super::requests::Mapping;

type Program = PortMapperProgram;

onc_rpc_program_procedure!(set(program: Mapping) -> bool);

pub use self::set::{Parameters, Procedure};
