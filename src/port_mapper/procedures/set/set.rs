use super::parameters::Parameters;
use super::super::super::program::PortMapperProgram;
use super::super::super::requests::RequestId;
use super::super::super::super::rpc::RpcProcedure;

pub struct Set;

impl RpcProcedure for Set {
    type Program = PortMapperProgram;
    type Parameters = Parameters;
    type ResultData = bool;

    fn procedure() -> u32 {
        RequestId::set.procedure()
    }
}
