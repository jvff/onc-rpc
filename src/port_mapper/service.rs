use super::procedures::ProcedureMessage;
use super::requests::{Request, Response};
use super::super::service::RpcServiceConfig;

pub struct PortMapperService;

impl RpcServiceConfig for PortMapperService {
    type Request = Request;
    type ProcedureMessage = ProcedureMessage;
    type Response = Response;
}
