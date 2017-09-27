use super::super::port_mapper_procedure::PortMapperProcedure;
use super::super::super::requests::{Mapping, RequestId};

pub struct Dump;

impl PortMapperProcedure for Dump {
    type ResultData = Vec<Mapping>;

    fn procedure(&self) -> u32 {
        RequestId::Dump.procedure()
    }
}
