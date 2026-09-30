use aws_lc_rs::error::Unspecified;

#[derive(Debug, PartialEq)]
pub(crate) enum SignatureCheck {
    LengthMismatch { expected: usize, found: usize },
    Matches,
    Mismatch(Unspecified),
}
