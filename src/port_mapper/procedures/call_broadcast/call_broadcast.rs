use super::parameters::Parameters;
use super::super::super::program::PortMapperProgram;
use super::super::super::requests::{CallResult, RequestId};
use super::super::super::super::rpc::RpcProcedure;

pub struct CallBroadcast;

impl RpcProcedure for CallBroadcast {
    type Program = PortMapperProgram;
    type Parameters = Parameters;
    type ResultData = CallResult;

    fn procedure() -> u32 {
        RequestId::CallBroadcast.procedure()
    }
}
