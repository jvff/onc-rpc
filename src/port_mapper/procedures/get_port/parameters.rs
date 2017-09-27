use super::super::super::requests::Mapping;

#[derive(Deserialize, Serialize)]
pub struct Parameters {
    mapping: Mapping,
}

impl From<Mapping> for Parameters {
    fn from(mapping: Mapping) -> Self {
        Parameters { mapping }
    }
}
