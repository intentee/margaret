use margaret_jws_verification::jws_rejection::JwsRejection;

pub enum AccessTokenVerification<TClaims> {
    Expired,
    MalformedClaims(serde_json::Error),
    NotReady,
    Rejected(JwsRejection),
    Verified(TClaims),
}
