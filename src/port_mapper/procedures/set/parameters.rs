use super::set::Set;
use super::super::super::requests::Mapping;
use super::super::super::super::rpc::RpcCall;

type Procedure = Set;

onc_rpc_program_procedure_parameters!(program: Mapping);
