#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeChallengeRejection {
    NotBase64Url,
    WrongDigestLength,
}
