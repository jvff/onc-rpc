use tokio_core::net::TcpStream;
use tokio_proto::multiplex;

use super::client_service::ClientService;
use super::super::record::RecordProtocol;

type RecordService = multiplex::ClientService<TcpStream, RecordProtocol>;

pub struct PortMapper {
    service: ClientService<RecordService>,
}

impl From<RecordService> for PortMapper {
    fn from(record_service: RecordService) -> Self {
        PortMapper {
            service: ClientService::from(record_service),
        }
    }
}
