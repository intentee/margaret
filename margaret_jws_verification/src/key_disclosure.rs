use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum KeyDisclosure {
    PrivateKey,
    SymmetricKey,
}

impl Display for KeyDisclosure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::PrivateKey => write!(formatter, "the key publishes its private key"),
            Self::SymmetricKey => write!(formatter, "the key publishes a symmetric secret"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeyDisclosure;

    #[test]
    fn describes_every_disclosure() {
        assert_eq!(
            KeyDisclosure::PrivateKey.to_string(),
            "the key publishes its private key"
        );
        assert_eq!(
            KeyDisclosure::SymmetricKey.to_string(),
            "the key publishes a symmetric secret"
        );
    }
}
