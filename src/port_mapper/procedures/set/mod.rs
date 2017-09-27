use super::port_mapper_procedure::PortMapperProcedure;
use super::super::requests::RequestId;

pub struct Set;

impl PortMapperProcedure for Set {
    type ResultData = bool;

    fn procedure(&self) -> u32 {
        RequestId::Set.procedure()
    }
}

