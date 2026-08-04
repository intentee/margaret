use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ProjectNameRejection {
    Empty,
    InvalidCharacter { character: char, index: usize },
    NonLetterStart { character: char },
}

impl Display for ProjectNameRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty => write!(formatter, "the project name is empty"),
            Self::InvalidCharacter { character, index } => write!(
                formatter,
                "the project name holds '{character}' at position {index}, but only lowercase ASCII letters, ASCII digits and underscores are allowed"
            ),
            Self::NonLetterStart { character } => write!(
                formatter,
                "the project name starts with '{character}', but it has to start with a lowercase ASCII letter"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectNameRejection;

    #[test]
    fn describes_an_empty_project_name() {
        assert_eq!(
            ProjectNameRejection::Empty.to_string(),
            "the project name is empty"
        );
    }

    #[test]
    fn describes_the_offending_character_and_its_position() {
        assert_eq!(
            ProjectNameRejection::InvalidCharacter {
                character: '-',
                index: 4,
            }
            .to_string(),
            "the project name holds '-' at position 4, but only lowercase ASCII letters, ASCII digits and underscores are allowed"
        );
    }

    #[test]
    fn describes_a_project_name_that_does_not_start_with_a_letter() {
        assert_eq!(
            ProjectNameRejection::NonLetterStart { character: '9' }.to_string(),
            "the project name starts with '9', but it has to start with a lowercase ASCII letter"
        );
    }
}
