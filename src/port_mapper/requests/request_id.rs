#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RequestId {
    Null,
    Set,
    Unset,
    GetPort,
    Dump,
    CallBroadcast,
}

impl RequestId {
    pub fn procedure(self) -> u32 {
        match self {
            RequestId::Null => 0,
            RequestId::Set => 1,
            RequestId::Unset => 2,
            RequestId::GetPort => 3,
            RequestId::Dump => 4,
            RequestId::CallBroadcast => 5,
        }
    }
}
