use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

fn starts_a_name(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn continues_a_name(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnvironmentVariableName {
    name: String,
}

impl EnvironmentVariableName {
    #[must_use]
    pub fn new(name: &str) -> Option<Self> {
        let mut characters = name.chars();

        if !characters.next().is_some_and(starts_a_name) {
            return None;
        }

        if !characters.all(continues_a_name) {
            return None;
        }

        Some(Self {
            name: name.to_string(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl Display for EnvironmentVariableName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::EnvironmentVariableName;

    fn accepts(name: &str) -> bool {
        EnvironmentVariableName::new(name).is_some()
    }

    #[test]
    fn accepts_a_screaming_snake_case_name() {
        let name = EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable");

        assert_eq!(name.as_str(), "DATABASE_URL");
        assert_eq!(name.to_string(), "DATABASE_URL");
    }

    #[test]
    fn accepts_a_name_that_starts_with_an_underscore() {
        assert!(accepts("_PRIVATE"));
    }

    #[test]
    fn accepts_a_name_that_contains_digits() {
        assert!(accepts("SERVER_2_PORT"));
    }

    #[test]
    fn accepts_a_lowercase_name() {
        assert!(accepts("database_url"));
    }

    #[test]
    fn rejects_an_empty_name() {
        assert!(!accepts(""));
    }

    #[test]
    fn rejects_a_name_that_starts_with_a_digit() {
        assert!(!accepts("2_SERVERS"));
    }

    #[test]
    fn rejects_a_name_that_contains_a_dash() {
        assert!(!accepts("DATABASE-URL"));
    }

    #[test]
    fn rejects_a_name_that_contains_an_equals_sign() {
        assert!(!accepts("DATABASE=URL"));
    }

    #[test]
    fn rejects_a_name_that_contains_a_non_ascii_character() {
        assert!(!accepts("DATABASE_ŁRL"));
    }
}
