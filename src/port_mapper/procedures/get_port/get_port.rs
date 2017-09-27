use super::parameters::Parameters;
use super::super::super::program::PortMapperProgram;
use super::super::super::requests::RequestId;
use super::super::super::super::rpc::RpcProcedure;

pub struct GetPort;

impl RpcProcedure for GetPort {
    type Program = PortMapperProgram;
    type Parameters = Parameters;
    type ResultData = u32;

    fn procedure(&self) -> u32 {
        RequestId::GetPort.procedure()
    }
}
