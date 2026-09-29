#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum KeySetComposition {
    IncludesEncryptionKeys,
    SignatureKeysOnly,
}
