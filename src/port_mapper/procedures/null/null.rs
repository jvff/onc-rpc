use super::super::port_mapper_procedure::PortMapperProcedure;
use super::super::super::requests::RequestId;

pub struct Null;

impl PortMapperProcedure for Null {
    type ResultData = ();

    fn procedure(&self) -> u32 {
        RequestId::Null.procedure()
    }
}
