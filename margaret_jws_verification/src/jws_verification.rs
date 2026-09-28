use crate::jws_rejection::JwsRejection;
use crate::verified_jws::VerifiedJws;

pub enum JwsVerification {
    Rejected(JwsRejection),
    Verified(VerifiedJws),
}
