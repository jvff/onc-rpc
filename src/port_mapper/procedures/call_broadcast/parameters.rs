use super::call_broadcast::CallBroadcast;
use super::super::super::requests::CallArgs;
use super::super::super::super::rpc::RpcCall;

type Procedure = CallBroadcast;

onc_rpc_program_procedure_parameters!(arguments: CallArgs);
