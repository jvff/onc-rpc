use std::marker::PhantomData;
use std::net::SocketAddr;

use tokio_proto::multiplex::Multiplex;
use tokio_proto::TcpServer;

use super::rpc_service_config::RpcServiceConfig;
use super::super::record::RecordProtocol;

pub struct RpcServer<P>
where
    P: RpcServiceConfig,
{
    pub server: TcpServer<Multiplex, RecordProtocol>,
    _service_parameters: PhantomData<P>,
}

impl<P> RpcServer<P>
where
    P: RpcServiceConfig + Send + Sync + 'static,
{
    pub fn new(address: SocketAddr) -> Self {
        RpcServer {
            server: TcpServer::new(RecordProtocol, address),
            _service_parameters: PhantomData,
        }
    }
}
