use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum Malformation {
    Unreadable { source: serde_json::Error },
}

impl Display for Malformation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Unreadable { source } => write!(
                formatter,
                "the request input could not be interpreted: {source}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Malformation;

    #[test]
    fn describes_an_unreadable_input_with_its_cause() {
        let source = serde_json::from_str::<u8>("\"text\"").expect_err("text is not a number");
        let cause = source.to_string();

        assert_eq!(
            Malformation::Unreadable { source }.to_string(),
            format!("the request input could not be interpreted: {cause}")
        );
    }
}
