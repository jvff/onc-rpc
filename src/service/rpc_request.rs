/// Represents an RPC request.
///
/// A type that implements this must give a hint on the type of the expected
/// response of a request message. Normally this trait is applied to an
/// enumeration of possible requests and has as a response hint an enumeration
/// mapping each request to a response type.
///
/// The hint is used during deserialization of the response message.
pub trait RpcRequest {
    /// The type of the hint of the expected response type.
    type ResponseHint: Copy;

    /// Getter method for the response type hint.
    fn response_hint(&self) -> Self::ResponseHint;
}
