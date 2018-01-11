use std::marker::PhantomData;
use std::net::SocketAddr;

use futures::{Async, Future, Poll};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::Connect as TcpConnect;
use tokio_proto::TcpClient;
use tokio_proto::multiplex::{ClientService, Multiplex};

use super::super::errors::{Error, ErrorKind, ResultExt};
use super::super::record::RecordProtocol;

/// An asynchronous connection attempt to a remote program.
///
/// The type parameter `T` is the asynchronous client interface to the remote
/// program. Connection is performed through TCP, and communication is through
/// the record protocol.
pub struct Connect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    address: SocketAddr,
    connect: TcpConnect<Multiplex, RecordProtocol>,
    _service: PhantomData<T>,
}

impl<T> Connect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    /// Create a new connection attempt to the remote program at the given
    /// `address` and using the given `handle` to the asynchronous event
    /// reactor.
    pub fn new(address: SocketAddr, handle: &Handle) -> Self {
        let client = TcpClient::new(RecordProtocol);

        Connect {
            address,
            connect: client.connect(&address, handle),
            _service: PhantomData,
        }
    }
}

impl<T> Future for Connect<T>
where
    T: From<ClientService<TcpStream, RecordProtocol>>,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll_result = self.connect.poll()
            .chain_err(|| {
                ErrorKind::ProgramConnectionError(self.address.to_string())
            });
        let record_service = try_ready!(poll_result);

        Ok(Async::Ready(T::from(record_service)))
    }
}
