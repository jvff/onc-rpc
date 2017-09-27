use super::dump::Dump;
use super::super::super::super::rpc::RpcCall;

#[derive(Clone, Deserialize, Serialize)]
pub struct Parameters;

impl Default for Parameters {
    fn default() -> Self {
        Parameters
    }
}

impl RpcCall for Parameters {
    type Procedure = Dump;

    fn parameters(&self) -> Parameters {
        self.clone()
    }
}
