use margaret_jws_verification::jws_rejection::JwsRejection;

pub enum JwksSecretVerificationResult<TClaims> {
    MalformedClaims(serde_json::Error),
    Rejected(JwsRejection),
    SignedWithCurrent(TClaims),
    SignedWithNextKey,
    SignedWithPrevious(TClaims),
}
