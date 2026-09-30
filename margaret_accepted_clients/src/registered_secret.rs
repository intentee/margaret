use aws_lc_rs::digest::SHA256_OUTPUT_LEN;

pub(crate) enum RegisteredSecret {
    Digest([u8; SHA256_OUTPUT_LEN]),
    Public,
}
