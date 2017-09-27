use std::net::SocketAddr;

use futures::{Async, Future, Poll};
use tokio_core::reactor::Handle;
use tokio_proto::{Connect, TcpClient};
use tokio_proto::multiplex::Multiplex;

use super::port_mapper::PortMapper;
use super::super::errors::{Error, ErrorKind, ResultExt};
use super::super::record::RecordProtocol;

pub struct PortMapperConnect<'a> {
    address: &'a SocketAddr,
    connect: Connect<Multiplex, RecordProtocol>,
}

impl<'a> PortMapperConnect<'a> {
    pub fn connect(address: &'a SocketAddr, handle: &Handle) -> Self {
        let client = TcpClient::new(RecordProtocol);

        PortMapperConnect {
            address,
            connect: client.connect(address, handle),
        }
    }
}

impl<'a> Future for PortMapperConnect<'a> {
    type Item = PortMapper;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll_result = self.connect.poll()
            .chain_err(|| {
                ErrorKind::PortMapperConnectionError(self.address.to_string())
            });
        let record_service = try_ready!(poll_result);

        Ok(Async::Ready(PortMapper::from(record_service)))
    }
}
