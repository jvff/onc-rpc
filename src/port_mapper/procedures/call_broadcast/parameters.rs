use super::super::super::requests::CallArgs;

#[derive(Deserialize, Serialize)]
pub struct Parameters {
    args: CallArgs,
}
