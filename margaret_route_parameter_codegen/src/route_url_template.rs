use crate::url_segment::UrlSegment;

#[must_use]
pub fn route_url_template(path: &str) -> Vec<UrlSegment> {
    let mut segments = Vec::new();
    let mut literal = String::new();
    let mut characters = path.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            '{' if characters.peek() == Some(&'{') => {
                characters.next();
                literal.push('{');
            }
            '}' if characters.peek() == Some(&'}') => {
                characters.next();
                literal.push('}');
            }
            '{' => {
                if !literal.is_empty() {
                    segments.push(UrlSegment::Literal(std::mem::take(&mut literal)));
                }

                let mut name = String::new();

                for inner in characters.by_ref() {
                    if inner == '}' {
                        break;
                    }

                    name.push(inner);
                }

                segments.push(match name.strip_prefix('*') {
                    Some(wildcard) => UrlSegment::WildcardParameter(wildcard.to_string()),
                    None => UrlSegment::Parameter(name),
                });
            }
            _ => literal.push(character),
        }
    }

    if !literal.is_empty() {
        segments.push(UrlSegment::Literal(literal));
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::route_url_template;
    use crate::url_segment::UrlSegment;

    fn describe(path: &str) -> Vec<String> {
        route_url_template(path)
            .into_iter()
            .map(|segment| match segment {
                UrlSegment::Literal(text) => format!("literal:{text}"),
                UrlSegment::Parameter(name) => format!("param:{name}"),
                UrlSegment::WildcardParameter(name) => format!("wildcard:{name}"),
            })
            .collect()
    }

    #[test]
    fn splits_a_literal_path() {
        assert_eq!(describe("/greeting"), vec!["literal:/greeting"]);
    }

    #[test]
    fn splits_a_named_parameter() {
        assert_eq!(
            describe("/articles/{article}"),
            vec!["literal:/articles/", "param:article"]
        );
    }

    #[test]
    fn distinguishes_a_catch_all_wildcard_from_a_named_parameter() {
        assert_eq!(
            describe("/files/{*rest}"),
            vec!["literal:/files/", "wildcard:rest"]
        );
    }

    #[test]
    fn unescapes_doubled_braces_into_literals() {
        assert_eq!(describe("/{{literal}}"), vec!["literal:/{literal}"]);
    }

    #[test]
    fn keeps_a_parameter_embedded_in_a_segment() {
        assert_eq!(
            describe("/images/img{id}.png"),
            vec!["literal:/images/img", "param:id", "literal:.png"]
        );
    }

    #[test]
    fn treats_an_unterminated_parameter_as_reaching_the_end() {
        assert_eq!(describe("/items/{id"), vec!["literal:/items/", "param:id"]);
    }

    #[test]
    fn splits_a_parameter_at_the_very_start() {
        assert_eq!(describe("{id}"), vec!["param:id"]);
    }
}
