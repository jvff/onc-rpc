use futures::{Future, IntoFuture};
use futures::future::{Flatten, FutureResult};
use serde_xdr;
use tokio_service::Service;

use super::call_future::CallFuture;
use super::requests::{Request, RequestId, RequestResult};
use super::super::errors::{Error, Result};
use super::super::message::RpcMessage;
use super::super::record::Record;

pub struct ClientService<S>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    Error: From<S::Error>,
{
    record_service: S,
}

impl<S> From<S> for ClientService<S>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    Error: From<S::Error>,
{
    fn from(record_service: S) -> Self {
        ClientService { record_service }
    }
}

impl<S> ClientService<S>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    Error: From<S::Error>,
{
    fn try_call(&self, request: Request) -> Result<CallFuture<S::Future>> {
        let result_hint = RequestId::from(&request);
        let rpc_message: RpcMessage<Request> = request.into();

        let request_record_bytes = serde_xdr::to_bytes(&rpc_message)?;
        let request_record = request_record_bytes.into();
        let result_record = self.record_service.call(request_record);

        Ok(CallFuture::new(result_hint, result_record))
    }
}

impl<S> Service for ClientService<S>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    Error: From<S::Error>,
{
    type Request = Request;
    type Response = RequestResult;
    type Error = Error;
    type Future = Flatten<FutureResult<CallFuture<S::Future>, Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.try_call(request).into_future().flatten()
    }
}
