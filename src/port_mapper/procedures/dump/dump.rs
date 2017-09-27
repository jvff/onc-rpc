use super::parameters::Parameters;
use super::super::super::program::PortMapperProgram;
use super::super::super::requests::{Mapping, RequestId};
use super::super::super::super::rpc::RpcProcedure;

pub struct Dump;

impl RpcProcedure for Dump {
    type Program = PortMapperProgram;
    type Parameters = Parameters;
    type ResultData = Vec<Mapping>;

    fn procedure(&self) -> u32 {
        RequestId::Dump.procedure()
    }
}
