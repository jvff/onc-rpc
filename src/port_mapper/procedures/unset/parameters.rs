use super::unset::Unset;
use super::super::super::requests::Mapping;
use super::super::super::super::rpc::RpcCall;

#[derive(Clone, Deserialize, Serialize)]
pub struct Parameters {
    mapping: Mapping,
}

impl From<Mapping> for Parameters {
    fn from(mapping: Mapping) -> Self {
        Parameters { mapping }
    }
}

impl RpcCall for Parameters {
    type Procedure = Unset;

    fn parameters(&self) -> Parameters {
        self.clone()
    }
}
