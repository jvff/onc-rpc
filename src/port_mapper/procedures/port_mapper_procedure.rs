use super::super::requests::Request;
use super::super::super::rpc::RpcProcedure;

pub trait PortMapperProcedure {
    type ResultData: Sized;

    fn procedure(&self) -> u32;
}

impl<T> RpcProcedure for T
where
    T: PortMapperProcedure
{
    type Parameters = Request;
    type ResultData = <Self as PortMapperProcedure>::ResultData;

    fn program(&self) -> u32 {
        100_000
    }

    fn version(&self) -> u32 {
        2
    }

    fn procedure(&self) -> u32 {
        PortMapperProcedure::procedure(self)
    }
}
