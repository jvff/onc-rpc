#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthStatus {
    Ok,
    BadCredentials,
    RejectedCredentials,
    BadVerifier,
    RejectedVerifier,
    TooWeak,
    InvalidResponse,
    UnknownFailure,
    KerberosGenericError,
    CredentialExpired,
    TicketFileProblem,
    DecodeFailure,
    WrongNetAddress,
}
