use std::collections::BTreeSet;
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

use margaret_attributes::is_snake_case_name::is_snake_case_name;
use margaret_url_path::admit_path_segment::admit_path_segment;
use margaret_url_path::admit_segment_text::admit_segment_text;
use margaret_url_path::encode_url_path_segment::encode_url_path_segment;
use margaret_url_path::path_segment::PathSegment;
use margaret_url_path::path_segment_admission::PathSegmentAdmission;
use margaret_url_path::path_segments::path_segments;
use margaret_url_path::path_separator::PATH_SEPARATOR;

use crate::route_path_error::RoutePathError;
use crate::url_segment::UrlSegment;

const CATCH_ALL_MARKER: char = '*';
const PARAMETER_CLOSE: char = '}';
const PARAMETER_OPEN: char = '{';

fn admitted_text(text: &str, admission: PathSegmentAdmission) -> Result<(), RoutePathError> {
    match admission {
        PathSegmentAdmission::Admitted => Ok(()),
        PathSegmentAdmission::Rejected(rejection) => Err(RoutePathError::UnroutableSegment {
            segment: text.to_string(),
            rejection,
        }),
    }
}

fn parameter_name(characters: &mut Peekable<Chars<'_>>) -> String {
    let mut name = String::new();

    while let Some(character) = characters.next() {
        match character {
            PARAMETER_OPEN | PARAMETER_CLOSE if characters.next_if_eq(&character).is_some() => {
                name.push(character);
            }
            PARAMETER_CLOSE => break,
            _ => name.push(character),
        }
    }

    name
}

fn pattern_segment(text: &str) -> PatternSegment {
    let mut characters = text.chars().peekable();
    let mut literal = String::new();

    while let Some(character) = characters.next() {
        match character {
            PARAMETER_OPEN | PARAMETER_CLOSE if characters.next_if_eq(&character).is_some() => {
                literal.push(character);
            }
            PARAMETER_OPEN => {
                return PatternSegment::Parameter {
                    name: parameter_name(&mut characters),
                    prefix: literal,
                };
            }
            _ => literal.push(character),
        }
    }

    PatternSegment::Literal(literal)
}

enum PatternSegment {
    Literal(String),
    Parameter { name: String, prefix: String },
}

struct TemplateAssembly {
    decoded: String,
    encoded: String,
    names: BTreeSet<String>,
    segments: Vec<UrlSegment>,
}

impl TemplateAssembly {
    fn declared_name(&mut self, name: &str) -> Result<String, RoutePathError> {
        if !is_snake_case_name(name) {
            return Err(RoutePathError::InvalidParameterName {
                name: name.to_string(),
            });
        }

        if !self.names.insert(name.to_string()) {
            return Err(RoutePathError::DuplicateParameterName {
                name: name.to_string(),
            });
        }

        Ok(name.to_string())
    }

    fn into_template(self) -> RouteUrlTemplate {
        let Self {
            decoded,
            encoded,
            mut segments,
            ..
        } = self;

        if segments.is_empty() {
            RouteUrlTemplate::Literal(encoded)
        } else {
            if !encoded.is_empty() {
                segments.push(UrlSegment::Literal { decoded, encoded });
            }

            RouteUrlTemplate::Parameterized(segments)
        }
    }

    fn push_literal(&mut self, literal: &str) -> Result<(), RoutePathError> {
        admitted_text(literal, admit_path_segment(literal))?;
        self.decoded.push_str(literal);
        self.encoded.push_str(&encode_url_path_segment(literal));

        Ok(())
    }

    fn push_parameter(&mut self, name: &str, prefix: String) -> Result<(), RoutePathError> {
        admitted_text(&prefix, admit_segment_text(&prefix))?;

        let segment = match name.strip_prefix(CATCH_ALL_MARKER) {
            Some(catch_all) => UrlSegment::CatchAllParameter {
                name: self.declared_name(catch_all)?,
                prefix,
            },
            None => UrlSegment::Parameter {
                name: self.declared_name(name)?,
                prefix,
            },
        };

        self.segments.push(UrlSegment::Literal {
            decoded: mem::take(&mut self.decoded),
            encoded: mem::take(&mut self.encoded),
        });
        self.segments.push(segment);

        Ok(())
    }

    fn push_separator(&mut self) {
        self.decoded.push(PATH_SEPARATOR);
        self.encoded.push(PATH_SEPARATOR);
    }

    fn push_segment(&mut self, segment: PathSegment<'_>) -> Result<(), RoutePathError> {
        match segment {
            PathSegment::Text(text) => match pattern_segment(text) {
                PatternSegment::Literal(literal) => self.push_literal(&literal),
                PatternSegment::Parameter { name, prefix } => self.push_parameter(&name, prefix),
            },
            PathSegment::TrailingSeparator => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RouteUrlTemplate {
    Literal(String),
    Parameterized(Vec<UrlSegment>),
}

impl RouteUrlTemplate {
    pub(crate) fn admitted(pattern: &str) -> Result<Self, RoutePathError> {
        let Some(path) = pattern.strip_prefix(PATH_SEPARATOR) else {
            return Err(RoutePathError::MissingLeadingSlash);
        };
        let mut assembly = TemplateAssembly {
            decoded: String::new(),
            encoded: String::new(),
            names: BTreeSet::new(),
            segments: Vec::new(),
        };

        assembly.push_separator();

        for (position, segment) in path_segments(path).enumerate() {
            if position > 0 {
                assembly.push_separator();
            }

            assembly.push_segment(segment)?;
        }

        Ok(assembly.into_template())
    }
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_segment_rejection::PathSegmentRejection;

    use super::RouteUrlTemplate;
    use crate::route_path_error::RoutePathError;
    use crate::route_path_registry::RoutePathRegistry;
    use crate::url_segment::UrlSegment;

    fn admission(pattern: &str) -> Result<RouteUrlTemplate, RoutePathError> {
        RoutePathRegistry::new()
            .admit_responder_path("public", pattern)
            .map(|path| path.template().clone())
    }

    fn parsed(pattern: &str) -> RouteUrlTemplate {
        admission(pattern).expect("the route path is admitted")
    }

    fn literal(decoded: &str, encoded: &str) -> UrlSegment {
        UrlSegment::Literal {
            decoded: decoded.to_string(),
            encoded: encoded.to_string(),
        }
    }

    fn parameter(prefix: &str, name: &str) -> UrlSegment {
        UrlSegment::Parameter {
            name: name.to_string(),
            prefix: prefix.to_string(),
        }
    }

    fn rejects_segment(pattern: &str, rejected: &str, expected: PathSegmentRejection) -> bool {
        matches!(
            admission(pattern),
            Err(RoutePathError::UnroutableSegment { segment, rejection })
                if segment == rejected && rejection == expected
        )
    }

    fn rejects_parameter_name(pattern: &str, rejected: &str) -> bool {
        matches!(
            admission(pattern),
            Err(RoutePathError::InvalidParameterName { name }) if name == rejected
        )
    }

    #[test]
    fn keeps_a_parameterless_path_as_one_literal_url_since_nothing_is_substituted() {
        assert_eq!(
            parsed("/greeting"),
            RouteUrlTemplate::Literal("/greeting".to_string())
        );
    }

    #[test]
    fn keeps_the_root_path_as_a_literal_url_instead_of_rejecting_its_trailing_separator() {
        assert_eq!(parsed("/"), RouteUrlTemplate::Literal("/".to_string()));
    }

    #[test]
    fn keeps_a_trailing_slash_in_the_literal_url_so_built_urls_reach_the_slashed_route() {
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
    fn percent_encodes_the_literals_around_a_parameter_so_built_urls_stay_valid() {
        assert_eq!(
            parsed("/caf\u{e9}s/{cafe}/m\u{e9}nu"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/caf\u{e9}s/", "/caf%C3%A9s/"),
                parameter("", "cafe"),
                literal("/m\u{e9}nu", "/m%C3%A9nu"),
            ])
        );
    }

    #[test]
    fn splits_a_named_parameter_from_its_literal_prefix_so_urls_substitute_its_value() {
        assert_eq!(
            parsed("/articles/{article}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/articles/", "/articles/"),
                parameter("", "article")
            ])
        );
    }

    #[test]
    fn accepts_a_parameter_named_after_a_rust_keyword_since_names_are_never_reserved() {
        assert_eq!(
            parsed("/kinds/{type}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/kinds/", "/kinds/"),
                parameter("", "type")
            ])
        );
    }

    #[test]
    fn distinguishes_a_catch_all_from_a_named_parameter() {
        assert_eq!(
            parsed("/files/{*rest}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/files/", "/files/"),
                UrlSegment::CatchAllParameter {
                    name: "rest".to_string(),
                    prefix: String::new(),
                },
            ])
        );
    }

    #[test]
    fn keeps_the_literal_text_preceding_a_parameter_so_urls_carry_the_segment_prefix() {
        assert_eq!(
            parsed("/images/img{id}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/images/", "/images/"),
                parameter("img", "id")
            ])
        );
    }

    #[test]
    fn unescapes_a_doubled_brace_preceding_a_parameter_like_the_router_does() {
        assert_eq!(
            parsed("/{{{id}"),
            RouteUrlTemplate::Parameterized(vec![literal("/", "/"), parameter("{", "id")])
        );
    }

    #[test]
    fn keeps_the_literal_text_preceding_a_catch_all_in_its_segment() {
        assert_eq!(
            parsed("/x{*rest}"),
            RouteUrlTemplate::Parameterized(vec![
                literal("/", "/"),
                UrlSegment::CatchAllParameter {
                    name: "rest".to_string(),
                    prefix: "x".to_string(),
                },
            ])
        );
    }

    #[test]
    fn rejects_a_path_without_a_leading_slash() {
        assert_eq!(admission("{id}"), Err(RoutePathError::MissingLeadingSlash));
    }

    #[test]
    fn rejects_an_empty_interior_segment_so_no_route_path_holds_a_double_slash() {
        assert!(rejects_segment(
            "/articles//comments",
            "",
            PathSegmentRejection::Empty
        ));
    }

    #[test]
    fn rejects_a_route_path_with_a_dot_segment_since_no_request_can_reach_it() {
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
    fn rejects_a_parameter_name_that_is_not_snake_case_like_every_declared_name() {
        assert!(rejects_parameter_name("/{article-id}", "article-id"));
    }

    #[test]
    fn rejects_a_catch_all_name_that_is_not_snake_case_like_every_declared_name() {
        assert!(rejects_parameter_name("/{*Rest}", "Rest"));
    }

    #[test]
    fn reads_an_escaped_closing_brace_into_the_parameter_name_like_the_router() {
        assert!(rejects_parameter_name("/{id}}}", "id}"));
    }

    #[test]
    fn reads_an_escaped_opening_brace_into_the_parameter_name_like_the_router() {
        assert!(rejects_parameter_name("/{a{{b}", "a{b"));
    }

    #[test]
    fn rejects_a_parameter_declared_twice_since_its_bound_value_would_be_ambiguous() {
        assert_eq!(
            admission("/{id}/{*id}"),
            Err(RoutePathError::DuplicateParameterName {
                name: "id".to_string(),
            })
        );
    }
}
