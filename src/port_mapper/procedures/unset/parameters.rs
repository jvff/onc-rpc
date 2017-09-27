use super::super::super::requests::Mapping;

#[derive(Deserialize, Serialize)]
pub struct Parameters {
    mapping: Mapping,
}
