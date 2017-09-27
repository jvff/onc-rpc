use super::call_broadcast::CallBroadcast;
use super::super::super::requests::CallArgs;
use super::super::super::super::rpc::RpcCall;

#[derive(Clone, Deserialize, Serialize)]
pub struct Parameters {
    call_arguments: CallArgs,
}

impl From<CallArgs> for Parameters {
    fn from(call_arguments: CallArgs) -> Self {
        Parameters { call_arguments }
    }
}

impl RpcCall for Parameters {
    type Procedure = CallBroadcast;

    fn parameters(&self) -> Parameters {
        self.clone()
    }
}
