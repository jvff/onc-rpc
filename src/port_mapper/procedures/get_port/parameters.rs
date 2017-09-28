use super::get_port::GetPort;
use super::super::super::requests::Mapping;
use super::super::super::super::rpc::RpcCall;

type Procedure = GetPort;

onc_rpc_program_procedure_parameters!(program: Mapping);
