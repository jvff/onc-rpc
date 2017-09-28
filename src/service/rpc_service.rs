use std::marker::PhantomData;

use futures::{Future, IntoFuture};
use futures::future::{Flatten, FutureResult};
use serde_xdr;
use tokio_service::Service;

use super::call_future::CallFuture;
use super::rpc_request::RpcRequest;
use super::rpc_service_config::RpcServiceConfig;
use super::try_from::TryFrom;
use super::super::errors::{Error, Result};
use super::super::record::Record;

pub struct RpcService<S, P>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    Error: From<S::Error>
        + From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    record_service: S,
    _service_parameters: PhantomData<P>,
}

impl<S, P> From<S> for RpcService<S, P>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    Error: From<S::Error>
        + From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    fn from(record_service: S) -> Self {
        RpcService {
            record_service,
            _service_parameters: PhantomData,
        }
    }
}

impl<S, P> RpcService<S, P>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    Error: From<S::Error>
        + From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    fn try_call(
        &self,
        request: P::Request,
    ) -> Result<CallFuture<S::Future, P>> {
        let response_hint = request.response_hint();
        let rpc_message: P::ProcedureMessage = request.into();

        let request_record_bytes = serde_xdr::to_bytes(&rpc_message)?;
        let request_record = request_record_bytes.into();
        let response_record = self.record_service.call(request_record);

        Ok(CallFuture::new(response_hint, response_record))
    }
}

impl<S, P> Service for RpcService<S, P>
where
    S: Service<Request = Record<Vec<u8>>, Response = Record<Vec<u8>>>,
    P: RpcServiceConfig,
    Error: From<S::Error>
        + From<<P::Response as TryFrom<P::ProcedureMessage>>::Error>,
{
    type Request = P::Request;
    type Response = P::Response;
    type Error = Error;
    type Future = Flatten<FutureResult<CallFuture<S::Future, P>, Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.try_call(request).into_future().flatten()
    }
}
