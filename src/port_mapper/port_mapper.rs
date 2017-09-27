use tokio_core::net::TcpStream;
use tokio_proto::multiplex;

use super::client_service::ClientService;
use super::super::record::RecordProtocol;

type RecordService = multiplex::ClientService<TcpStream, RecordProtocol>;

pub struct PortMapper {
    service: ClientService<RecordService>,
}
