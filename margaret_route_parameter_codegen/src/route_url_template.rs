use std::mem;

use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_url_path::admit_path_segment::admit_path_segment;
use margaret_url_path::admit_segment_text::admit_segment_text;
use margaret_url_path::encode_url_path_literal::encode_url_path_literal;
use margaret_url_path::path_segment_admission::PathSegmentAdmission;

use crate::route_path_error::RoutePathError;
use crate::url_segment::UrlSegment;

const CATCH_ALL_MARKER: char = '*';
const PARAMETER_CLOSE: char = '}';
const PARAMETER_OPEN: char = '{';
const PATH_SEPARATOR: char = '/';

fn admitted(text: &str, admission: PathSegmentAdmission) -> Result<(), RoutePathError> {
    match admission {
        PathSegmentAdmission::Admitted => Ok(()),
        PathSegmentAdmission::Rejected(rejection) => Err(RoutePathError::UnroutableSegment {
            segment: text.to_string(),
            rejection,
        }),
    }
}

fn parameter_name(name: &str) -> Result<String, RoutePathError> {
    if is_snake_case_identifier(name) {
        Ok(name.to_string())
    } else {
        Err(RoutePathError::InvalidParameterName {
            name: name.to_string(),
        })
    }
}

fn pattern_segments(path: &str) -> PatternSegments {
    let mut leading = Vec::new();
    let mut segment = PatternSegment::default();
    let mut characters = path.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            PARAMETER_OPEN if characters.peek() == Some(&PARAMETER_OPEN) => {
                characters.next();
                segment.push_literal(PARAMETER_OPEN);
            }
            PARAMETER_CLOSE if characters.peek() == Some(&PARAMETER_CLOSE) => {
                characters.next();
                segment.push_literal(PARAMETER_CLOSE);
            }
            PARAMETER_OPEN => segment.parameters.push(SegmentParameter {
                name: characters
                    .by_ref()
                    .take_while(|inner| *inner != PARAMETER_CLOSE)
                    .collect(),
                trailing: String::new(),
            }),
            PATH_SEPARATOR => leading.push(mem::take(&mut segment)),
            _ => segment.push_literal(character),
        }
    }

    PatternSegments {
        last: segment,
        leading,
    }
}

#[derive(Default)]
struct PatternSegment {
    leading: String,
    parameters: Vec<SegmentParameter>,
}

impl PatternSegment {
    fn push_literal(&mut self, character: char) {
        match self.parameters.last_mut() {
            Some(parameter) => parameter.trailing.push(character),
            None => self.leading.push(character),
        }
    }
}

struct PatternSegments {
    last: PatternSegment,
    leading: Vec<PatternSegment>,
}

struct SegmentParameter {
    name: String,
    trailing: String,
}

struct TemplateAssembly {
    literal: String,
    segments: Vec<UrlSegment>,
}

impl TemplateAssembly {
    fn into_template(self) -> RouteUrlTemplate {
        let Self {
            literal,
            mut segments,
        } = self;

        if segments.is_empty() {
            RouteUrlTemplate::Literal(literal)
        } else {
            if !literal.is_empty() {
                segments.push(UrlSegment::Literal(literal));
            }

            RouteUrlTemplate::Parameterized(segments)
        }
    }

    fn push_literal_segment(&mut self, text: &str, is_last: bool) -> Result<(), RoutePathError> {
        if !(text.is_empty() && is_last) {
            admitted(text, admit_path_segment(text))?;
        }

        self.literal.push_str(&encode_url_path_literal(text));

        Ok(())
    }

    fn push_parameter(
        &mut self,
        prefix: &str,
        SegmentParameter { name, trailing }: &SegmentParameter,
        is_last: bool,
    ) -> Result<(), RoutePathError> {
        admitted(prefix, admit_segment_text(prefix))?;
        admitted(trailing, admit_segment_text(trailing))?;

        let segment = match name.strip_prefix(CATCH_ALL_MARKER) {
            Some(catch_all) if is_last && trailing.is_empty() => UrlSegment::CatchAllParameter {
                name: parameter_name(catch_all)?,
                prefix: prefix.to_string(),
            },
            Some(catch_all) => {
                return Err(RoutePathError::CatchAllNotLast {
                    name: catch_all.to_string(),
                });
            }
            None => UrlSegment::Parameter {
                name: parameter_name(name)?,
                prefix: prefix.to_string(),
                suffix: trailing.clone(),
            },
        };

        self.segments
            .push(UrlSegment::Literal(mem::take(&mut self.literal)));
        self.segments.push(segment);

        Ok(())
    }

    fn push_segment(
        &mut self,
        PatternSegment {
            leading,
            parameters,
        }: &PatternSegment,
        is_last: bool,
    ) -> Result<(), RoutePathError> {
        match parameters.as_slice() {
            [] => self.push_literal_segment(leading, is_last),
            [parameter] => self.push_parameter(leading, parameter, is_last),
            [first, second, ..] => Err(RoutePathError::ParametersShareSegment {
                first: first.name.clone(),
                second: second.name.clone(),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RouteUrlTemplate {
    Literal(String),
    Parameterized(Vec<UrlSegment>),
}

impl RouteUrlTemplate {
    /// # Errors
    ///
    /// Returns `RoutePathError` when the pattern does not start with '/', holds a segment no
    /// request can reach, names a parameter with anything but a `snake_case` identifier, places
    /// two parameters in one segment, or places a catch-all parameter before the end.
    pub fn parse(pattern: &str) -> Result<Self, RoutePathError> {
        let Some(path) = pattern.strip_prefix(PATH_SEPARATOR) else {
            return Err(RoutePathError::MissingLeadingSlash);
        };
        let PatternSegments { last, leading } = pattern_segments(path);
        let mut assembly = TemplateAssembly {
            literal: PATH_SEPARATOR.to_string(),
            segments: Vec::new(),
        };

        for segment in &leading {
            assembly.push_segment(segment, false)?;
            assembly.literal.push(PATH_SEPARATOR);
        }

        assembly.push_segment(&last, true)?;

        Ok(assembly.into_template())
    }
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_segment_rejection::PathSegmentRejection;

    use super::RouteUrlTemplate;
    use crate::route_path_error::RoutePathError;
    use crate::url_segment::UrlSegment;

    fn parsed(pattern: &str) -> RouteUrlTemplate {
        RouteUrlTemplate::parse(pattern).expect("the route path is routable")
    }

    fn literal(text: &str) -> UrlSegment {
        UrlSegment::Literal(text.to_string())
    }

    fn parameter(prefix: &str, name: &str, suffix: &str) -> UrlSegment {
        UrlSegment::Parameter {
            name: name.to_string(),
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
        }
    }

    fn rejects_segment(pattern: &str, rejected: &str, expected: PathSegmentRejection) -> bool {
        matches!(
            RouteUrlTemplate::parse(pattern),
            Err(RoutePathError::UnroutableSegment { segment, rejection })
                if segment == rejected && rejection == expected
        )
    }

    #[test]
    fn keeps_a_literal_path() {
        assert_eq!(
            parsed("/greeting"),
            RouteUrlTemplate::Literal("/greeting".to_string())
        );
    }

    #[test]
    fn keeps_the_root_path() {
        assert_eq!(parsed("/"), RouteUrlTemplate::Literal("/".to_string()));
    }

    #[test]
    fn keeps_a_trailing_slash() {
        assert_eq!(
            parsed("/articles/"),
            RouteUrlTemplate::Literal("/articles/".to_string())
        );
    }

    #[test]
    fn encodes_doubled_braces_into_literal_url_text() {
        assert_eq!(
            parsed("/{{literal}}"),
            RouteUrlTemplate::Literal("/%7Bliteral%7D".to_string())
        );
    }

    #[test]
    fn encodes_the_literal_around_a_parameter() {
        assert_eq!(
            parsed("/caf\u{e9}s/{cafe}/m\u{e9}nu"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/caf%C3%A9s/"),
                parameter("", "cafe", ""),
                literal("/m%C3%A9nu"),
            ])
        );
    }

    #[test]
    fn splits_a_named_parameter() {
        assert_eq!(
            parsed("/articles/{article}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/articles/"),
                parameter("", "article", "")
            ])
        );
    }

    #[test]
    fn distinguishes_a_catch_all_from_a_named_parameter() {
        assert_eq!(
            parsed("/files/{*rest}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/files/"),
                UrlSegment::CatchAllParameter {
                    name: "rest".to_string(),
                    prefix: String::new(),
                },
            ])
        );
    }

    #[test]
    fn keeps_the_literal_text_sharing_a_segment_with_a_parameter() {
        assert_eq!(
            parsed("/images/img{id}.png"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/images/"),
                parameter("img", "id", ".png")
            ])
        );
    }

    #[test]
    fn keeps_the_literal_text_preceding_a_catch_all_in_its_segment() {
        assert_eq!(
            parsed("/x{*rest}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/"),
                UrlSegment::CatchAllParameter {
                    name: "rest".to_string(),
                    prefix: "x".to_string(),
                },
            ])
        );
    }

    #[test]
    fn treats_an_unterminated_parameter_as_reaching_the_end() {
        assert_eq!(
            parsed("/items/{id"),
            RouteUrlTemplate::Parameterized(vec![literal("/items/"), parameter("", "id", "")])
        );
    }

    #[test]
    fn rejects_a_path_without_a_leading_slash() {
        assert_eq!(
            RouteUrlTemplate::parse("{id}"),
            Err(RoutePathError::MissingLeadingSlash)
        );
    }

    #[test]
    fn rejects_an_empty_interior_segment() {
        assert!(rejects_segment(
            "/articles//comments",
            "",
            PathSegmentRejection::Empty
        ));
    }

    #[test]
    fn rejects_a_dot_segment() {
        assert!(rejects_segment(
            "/articles/../admin",
            "..",
            PathSegmentRejection::DotSegment
        ));
    }

    #[test]
    fn rejects_a_control_character_in_a_literal_segment() {
        assert!(rejects_segment(
            "/a\nb",
            "a\nb",
            PathSegmentRejection::ControlCharacter
        ));
    }

    #[test]
    fn rejects_a_control_character_before_a_parameter() {
        assert!(rejects_segment(
            "/a\t{id}",
            "a\t",
            PathSegmentRejection::ControlCharacter
        ));
    }

    #[test]
    fn rejects_a_control_character_after_a_parameter() {
        assert!(rejects_segment(
            "/{id}\t",
            "\t",
            PathSegmentRejection::ControlCharacter
        ));
    }

    #[test]
    fn rejects_two_parameters_in_one_segment() {
        assert!(matches!(
            RouteUrlTemplate::parse("/{first}-{second}"),
            Err(RoutePathError::ParametersShareSegment { first, second })
                if first == "first" && second == "second"
        ));
    }

    #[test]
    fn rejects_a_catch_all_followed_by_another_segment() {
        assert!(matches!(
            RouteUrlTemplate::parse("/{*rest}/tail"),
            Err(RoutePathError::CatchAllNotLast { name }) if name == "rest"
        ));
    }

    #[test]
    fn rejects_a_catch_all_followed_by_literal_text() {
        assert!(matches!(
            RouteUrlTemplate::parse("/{*rest}.png"),
            Err(RoutePathError::CatchAllNotLast { name }) if name == "rest"
        ));
    }

    #[test]
    fn rejects_a_parameter_name_that_is_not_an_identifier() {
        assert!(matches!(
            RouteUrlTemplate::parse("/{article-id}"),
            Err(RoutePathError::InvalidParameterName { name }) if name == "article-id"
        ));
    }

    #[test]
    fn rejects_a_catch_all_name_that_is_not_an_identifier() {
        assert!(matches!(
            RouteUrlTemplate::parse("/{*}"),
            Err(RoutePathError::InvalidParameterName { name }) if name.is_empty()
        ));
    }
}
