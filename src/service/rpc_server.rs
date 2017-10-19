use std::marker::PhantomData;
use std::net::SocketAddr;

use tokio_proto::multiplex::Multiplex;
use tokio_proto::TcpServer;
use tokio_service::NewService;

use super::rpc_service_config::RpcServiceConfig;
use super::super::errors::Error;
use super::super::record::{Record, RecordProtocol};

pub struct RpcServer<P>
where
    P: RpcServiceConfig,
{
    server: TcpServer<Multiplex, RecordProtocol>,
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

    pub fn serve_service<S>(&self, service: S)
    where
        S: 'static + NewService + Send + Sync,
        S::Request: From<Record<Vec<u8>>>,
        S::Error: Into<Error>,
        Record<Vec<u8>>: From<S::Response>,
    {
        self.server.serve(service);
    }
}
