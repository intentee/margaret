use aws_lc_rs::signature::ParsedPublicKey;

#[derive(Clone)]
pub struct Rs256PublicKey {
    pub(crate) key: ParsedPublicKey,
}
