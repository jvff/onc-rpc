use super::super::port_mapper_procedure::PortMapperProcedure;
use super::super::super::requests::RequestId;

pub struct Unset;

impl PortMapperProcedure for Unset {
    type ResultData = bool;

    fn procedure(&self) -> u32 {
        RequestId::Unset.procedure()
    }
}
