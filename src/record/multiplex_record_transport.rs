use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::RequestId;

use super::record::Record;
use super::record_codec::RecordCodec;
use super::super::errors::Error;

pub struct MultiplexRecordTransport<T>
where
    T: AsyncRead + AsyncWrite,
{
    transport: Framed<T, RecordCodec>,
}

impl<T> MultiplexRecordTransport<T>
where
    T: AsyncRead + AsyncWrite,
{
    pub fn new(connection: T) -> Self {
        MultiplexRecordTransport {
            transport: connection.framed(RecordCodec),
        }
    }
}

impl<T> Sink for MultiplexRecordTransport<T>
where
    T: AsyncRead + AsyncWrite,
{
    type SinkItem = (RequestId, Record<Vec<u8>>);
    type SinkError = Error;

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        let (transaction_id, mut record) = item;

        record.set_transaction_id(transaction_id as u32);

        match self.transport.start_send(record)? {
            AsyncSink::Ready => Ok(AsyncSink::Ready),
            AsyncSink::NotReady(record) => {
                Ok(AsyncSink::NotReady((transaction_id, record)))
            }
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        try_ready!(self.transport.poll_complete());

        Ok(Async::Ready(()))
    }
}

impl<T> Stream for MultiplexRecordTransport<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Item = (RequestId, Record<Vec<u8>>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match try_ready!(self.transport.poll()) {
            Some(record) => {
                Ok(Async::Ready(
                    Some((record.transaction_id().into(), record))
                ))
            }
            None => Ok(Async::Ready(None)),
        }
    }
}
