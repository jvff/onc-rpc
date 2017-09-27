use super::port_mapper_procedure::PortMapperProcedure;
use super::super::requests::{CallResult, RequestId};

pub struct CallBroadcast;

impl PortMapperProcedure for CallBroadcast {
    type ResultData = CallResult;

    fn procedure(&self) -> u32 {
        RequestId::CallBroadcast.procedure()
    }
}

