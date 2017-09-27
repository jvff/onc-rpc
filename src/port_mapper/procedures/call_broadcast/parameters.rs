use super::super::super::requests::CallArgs;

#[derive(Deserialize, Serialize)]
pub struct Parameters {
    args: CallArgs,
}

impl From<CallArgs> for Parameters {
    fn from(call_args: CallArgs) -> Self {
        Parameters { call_args }
    }
}
