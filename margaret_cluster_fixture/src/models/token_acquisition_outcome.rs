pub enum TokenAcquisitionOutcome {
    Acquired,
    Refused,
    Unavailable,
}

impl TokenAcquisitionOutcome {
    #[must_use]
    pub fn stored(&self) -> &'static str {
        match self {
            Self::Acquired => "Acquired",
            Self::Refused => "Refused",
            Self::Unavailable => "Unavailable",
        }
    }
}
