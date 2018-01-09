use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::multiplex::{ClientProto, ServerProto};

use super::multiplex_record_transport::MultiplexRecordTransport;
use super::record::Record;
use super::super::errors::{Error, Result};

/// A protocol that serializes its messages using the
/// [Record Marking Standard][record].
///
/// [record]: https://tools.ietf.org/html/rfc1057#page-18
pub struct RecordProtocol;

impl<T> ClientProto<T> for RecordProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Transport = MultiplexRecordTransport<T>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(MultiplexRecordTransport::new(io))
    }
}

impl<T> ServerProto<T> for RecordProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = Record<Vec<u8>>;
    type Response = Record<Vec<u8>>;
    type Error = Error;
    type Transport = MultiplexRecordTransport<T>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(MultiplexRecordTransport::new(io))
    }
}
