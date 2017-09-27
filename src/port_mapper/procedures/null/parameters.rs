use super::null::Null;
use super::super::super::super::rpc::RpcCall;

#[derive(Clone, Deserialize, Serialize)]
pub struct Parameters;

impl Default for Parameters {
    fn default() -> Self {
        Parameters
    }
}

impl RpcCall for Parameters {
    type Procedure = Null;

    fn parameters(&self) -> Parameters {
        self.clone()
    }
}
