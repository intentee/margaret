#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeVerifierRejection {
    ReservedCharacter { position: usize },
    TooLong { length: usize },
    TooShort { length: usize },
}
