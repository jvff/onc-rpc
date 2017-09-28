use std::io::Cursor;
use std::marker::PhantomData;

use futures::{Async, Future, Poll};
use serde_xdr::Deserializer;

use super::deserialize_with_hint::DeserializeWithHint;
use super::rpc_request::RpcRequest;
use super::rpc_service_config::RpcServiceConfig;
use super::try_from::TryFrom;
use super::super::errors::Error;
use super::super::record::Record;

pub struct CallFuture<R, P>
where
    R: Future<Item = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    R::Error: Into<Error>,
    Error: From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    response_hint: <P::Request as RpcRequest>::ResponseHint,
    response_record: R,
    _service_parameters: PhantomData<P>,
}

impl<R, P> CallFuture<R, P>
where
    R: Future<Item = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    R::Error: Into<Error>,
    Error: From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    pub fn new(
        response_hint: <P::Request as RpcRequest>::ResponseHint,
        response_record: R,
    ) -> Self {
        CallFuture {
            response_hint,
            response_record,
            _service_parameters: PhantomData,
        }
    }
}

impl<R, P> Future for CallFuture<R, P>
where
    R: Future<Item = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    R::Error: Into<Error>,
    Error: From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    type Item = P::Response;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let response_record = match self.response_record.poll() {
            Ok(Async::Ready(record)) => record,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(error) => return Err(error.into()),
        };

        let mut reader = Cursor::new(response_record);
        let mut deserializer = Deserializer::new(&mut reader);

        let hint = self.response_hint;
        let response_message: P::ProcedureMessage =
            DeserializeWithHint::deserialize_with_hint(
                hint,
                &mut deserializer,
            )?;

        Ok(Async::Ready(TryFrom::try_from(response_message)?))
    }
}
