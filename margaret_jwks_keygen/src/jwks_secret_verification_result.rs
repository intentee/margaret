pub enum JwksSecretVerificationResult<TClaims> {
    Invalid,
    SignedWithCurrent(TClaims),
    SignedWithPrevious(TClaims),
}
