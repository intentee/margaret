use crate::project_name::ProjectName;
use crate::project_name_outcome::ProjectNameOutcome;
use crate::project_name_rejection::ProjectNameRejection;

fn is_allowed_inside(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
}

#[must_use]
pub(crate) fn parse_project_name(candidate: &str) -> ProjectNameOutcome {
    let mut characters = candidate.char_indices();

    let Some((_, first)) = characters.next() else {
        return ProjectNameOutcome::Rejected(ProjectNameRejection::Empty);
    };

    if !first.is_ascii_lowercase() {
        return ProjectNameOutcome::Rejected(ProjectNameRejection::NonLetterStart {
            character: first,
        });
    }

    for (index, character) in characters {
        if !is_allowed_inside(character) {
            return ProjectNameOutcome::Rejected(ProjectNameRejection::InvalidCharacter {
                character,
                index,
            });
        }
    }

    ProjectNameOutcome::Accepted(ProjectName::new(candidate.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_project_name;
    use crate::project_name::ProjectName;
    use crate::project_name_outcome::ProjectNameOutcome;
    use crate::project_name_rejection::ProjectNameRejection;

    #[test]
    fn accepts_a_name_built_from_letters_digits_and_underscores() {
        assert_eq!(
            parse_project_name("acme_2"),
            ProjectNameOutcome::Accepted(ProjectName::new("acme_2".to_string()))
        );
    }

    #[test]
    fn rejects_an_empty_name() {
        assert_eq!(
            parse_project_name(""),
            ProjectNameOutcome::Rejected(ProjectNameRejection::Empty)
        );
    }

    #[test]
    fn rejects_a_name_that_starts_with_a_digit() {
        assert_eq!(
            parse_project_name("9acme"),
            ProjectNameOutcome::Rejected(ProjectNameRejection::NonLetterStart { character: '9' })
        );
    }

    #[test]
    fn rejects_a_name_that_starts_with_an_underscore() {
        assert_eq!(
            parse_project_name("_acme"),
            ProjectNameOutcome::Rejected(ProjectNameRejection::NonLetterStart { character: '_' })
        );
    }

    #[test]
    fn rejects_an_uppercase_letter_inside_the_name() {
        assert_eq!(
            parse_project_name("acmeCorp"),
            ProjectNameOutcome::Rejected(ProjectNameRejection::InvalidCharacter {
                character: 'C',
                index: 4,
            })
        );
    }

    #[test]
    fn rejects_a_dash_inside_the_name() {
        assert_eq!(
            parse_project_name("acme-corp"),
            ProjectNameOutcome::Rejected(ProjectNameRejection::InvalidCharacter {
                character: '-',
                index: 4,
            })
        );
    }
}
