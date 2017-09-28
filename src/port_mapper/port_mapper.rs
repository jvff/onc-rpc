use std::net::SocketAddr;

use futures::future::{Flatten, FutureResult};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::multiplex;
use tokio_service::Service;

use super::get_port_result::GetPortResult;
use super::port_mapper_connect::PortMapperConnect;
use super::requests::{Mapping, Protocol, Request};
use super::service::PortMapperService;
use super::super::errors::Error;
use super::super::record::RecordProtocol;
use super::super::service::{CallFuture, RpcService};

type CallResultFuture = Flatten<FutureResult<CallFuture<RecordFuture, PortMapperService>, Error>>;
type RecordFuture = multiplex::ClientFuture<TcpStream, RecordProtocol>;
type RecordService = multiplex::ClientService<TcpStream, RecordProtocol>;

pub struct PortMapper {
    service: RpcService<RecordService, PortMapperService>,
}

impl PortMapper {
    pub fn connect<'a>(
        address: &'a SocketAddr,
        handle: &Handle,
    ) -> PortMapperConnect<'a> {
        PortMapperConnect::connect(address, handle)
    }

    pub fn get_port(
        &self,
        program: u32,
        version: u32,
        protocol: Protocol,
    ) -> GetPortResult<CallResultFuture> {
        let argument = Mapping {
            program,
            version,
            protocol,
            port: 0,
        };

        let request = Request::GetPort(argument);

        self.service.call(request).into()
    }
}

impl From<RecordService> for PortMapper {
    fn from(record_service: RecordService) -> Self {
        PortMapper {
            service: RpcService::from(record_service),
        }
    }
}
