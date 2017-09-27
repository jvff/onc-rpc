use std::net::SocketAddr;

use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::multiplex;

use super::client_service::ClientService;
use super::port_mapper_connect::PortMapperConnect;
use super::super::record::RecordProtocol;

type RecordService = multiplex::ClientService<TcpStream, RecordProtocol>;

pub struct PortMapper {
    service: ClientService<RecordService>,
}

impl PortMapper {
    pub fn connect<'a>(
        address: &'a SocketAddr,
        handle: &Handle,
    ) -> PortMapperConnect<'a> {
        PortMapperConnect::connect(address, handle)
    }
}

impl From<RecordService> for PortMapper {
    fn from(record_service: RecordService) -> Self {
        PortMapper {
            service: ClientService::from(record_service),
        }
    }
}
