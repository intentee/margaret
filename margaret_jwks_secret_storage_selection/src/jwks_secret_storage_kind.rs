use clap::ValueEnum;
use clap::builder::PossibleValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JwksSecretStorageKind {
    File,
    Memory,
}

impl ValueEnum for JwksSecretStorageKind {
    fn value_variants<'variants>() -> &'variants [Self] {
        &[Self::File, Self::Memory]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::File => PossibleValue::new("file"),
            Self::Memory => PossibleValue::new("memory"),
        })
    }
}
