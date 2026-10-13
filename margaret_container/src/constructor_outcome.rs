#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructorOutcome {
    Fallible,
    Infallible,
}

impl ConstructorOutcome {
    #[must_use]
    pub fn combined_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Infallible, Self::Infallible) => Self::Infallible,
            (Self::Fallible, _) | (_, Self::Fallible) => Self::Fallible,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConstructorOutcome;

    #[test]
    fn stays_infallible_only_when_both_constructions_are_infallible() {
        assert_eq!(
            ConstructorOutcome::Infallible.combined_with(ConstructorOutcome::Infallible),
            ConstructorOutcome::Infallible
        );
        assert_eq!(
            ConstructorOutcome::Infallible.combined_with(ConstructorOutcome::Fallible),
            ConstructorOutcome::Fallible
        );
        assert_eq!(
            ConstructorOutcome::Fallible.combined_with(ConstructorOutcome::Infallible),
            ConstructorOutcome::Fallible
        );
    }
}
