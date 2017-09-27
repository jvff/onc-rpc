use super::parameters::Parameters;
use super::super::super::program::PortMapperProgram;
use super::super::super::requests::RequestId;
use super::super::super::super::rpc::RpcProcedure;

pub struct Null;

impl RpcProcedure for Null {
    type Program = PortMapperProgram;
    type Parameters = Parameters;
    type ResultData = ();

    fn procedure() -> u32 {
        RequestId::Null.procedure()
    }
}
