use std::marker::PhantomData;
use std::net::SocketAddr;

use tokio_proto::multiplex::Multiplex;
use tokio_proto::TcpServer;
use tokio_service::{NewService, Service};

use super::rpc_server_service::RpcServerService;
use super::rpc_service_config::RpcServiceConfig;
use super::try_from::TryFrom;
use super::super::errors::Error;
use super::super::record::RecordProtocol;

/// A TCP server for serving RPC services.
///
/// The services must be compatible with the given RPC service configuration.
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
    <P::Request as TryFrom<P::ProcedureMessage>>::Error: Into<Error>,
{
    /// Create a new server instance.
    pub fn new(address: SocketAddr) -> Self {
        RpcServer {
            server: TcpServer::new(RecordProtocol, address),
            _service_parameters: PhantomData,
        }
    }

    /// Synchronously serve a given service.
    pub fn serve_rpc_service<S>(&self, service: S)
    where
        S: 'static
            + NewService<
                Request = P::Request,
                Response = P::Response,
                Instance = S,
            >
            + Service<
                Request = P::Request,
                Response = P::Response,
                Error = <S as NewService>::Error,
            >
            + Send
            + Sync,
        <S as Service>::Error: Into<Error>,
        <S as NewService>::Error: Into<Error>,
        Error: From<<P::Request as TryFrom<P::ProcedureMessage>>::Error>,
    {
        let rpc_service: RpcServerService<_, P> = service.into();

        self.server.serve(rpc_service);
    }
}
