use aws_lc_rs::aead::NONCE_LEN;

#[derive(Clone)]
pub struct SealedDocument {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; NONCE_LEN],
}
