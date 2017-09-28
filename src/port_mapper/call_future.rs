use std::io::Cursor;

use futures::{Async, Future, Poll};
use serde_xdr::Deserializer;

use super::procedures::ProcedureMessage;
use super::requests::{RequestId, RequestResult};
use super::super::errors::Error;
use super::super::record::Record;
use super::super::service::TryFrom;

pub struct CallFuture<R> {
    result_hint: RequestId,
    result_record: R,
}

impl<R> CallFuture<R> {
    pub fn new(result_hint: RequestId, result_record: R) -> Self {
        CallFuture { result_hint, result_record }
    }
}

impl<R> Future for CallFuture<R>
where
    R: Future<Item = Record<Vec<u8>>>,
    R::Error: Into<Error>,
{
    type Item = RequestResult;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let result_record = match self.result_record.poll() {
            Ok(Async::Ready(record)) => record,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(error) => return Err(error.into()),
        };

        let mut reader = Cursor::new(result_record);
        let mut deserializer = Deserializer::new(&mut reader);

        let hint = self.result_hint;
        let reply =
            ProcedureMessage::deserialize_with_hint(hint, &mut deserializer)?;

        Ok(Async::Ready(RequestResult::try_from(reply)?))
    }
}
