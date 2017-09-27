use super::super::port_mapper_procedure::PortMapperProcedure;
use super::super::super::requests::RequestId;

pub struct GetPort;

impl PortMapperProcedure for GetPort {
    type ResultData = u32;

    fn procedure(&self) -> u32 {
        RequestId::GetPort.procedure()
    }
}
