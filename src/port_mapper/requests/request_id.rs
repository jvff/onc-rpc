use super::request::Request;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RequestId {
    Null,
    Set,
    Unset,
    GetPort,
    Dump,
    CallBroadcast,
}

impl From<Request> for RequestId {
    fn from(request: Request) -> Self {
        RequestId::from(&request)
    }
}

impl<'a> From<&'a Request> for RequestId {
    fn from(request: &'a Request) -> Self {
        match *request {
            Request::Null => RequestId::Null,
            Request::Set(_) =>  RequestId::Set,
            Request::Unset(_) => RequestId::Unset,
            Request::GetPort(_) =>  RequestId::GetPort,
            Request::Dump => RequestId::Dump,
            Request::CallBroadcast(_) => RequestId::CallBroadcast,
        }
    }
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
