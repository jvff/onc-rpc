pub trait RpcRequest {
    type ResponseHint: Copy;

    fn response_hint(&self) -> Self::ResponseHint;
}
