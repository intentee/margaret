use crate::route_url_error::RouteUrlError;

pub fn reject_route_path_segment(parameter: &str, value: &str) -> Result<(), RouteUrlError> {
    match value {
        "." | ".." => Err(RouteUrlError::ReservedPathSegment {
            parameter: parameter.to_string(),
            value: value.to_string(),
        }),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::reject_route_path_segment;
    use crate::route_url_error::RouteUrlError;

    #[test]
    fn accepts_an_ordinary_identifier() {
        assert!(reject_route_path_segment("article", "rust").is_ok());
        assert!(reject_route_path_segment("article", "v1.2").is_ok());
    }

    #[test]
    fn accepts_a_value_that_only_starts_with_dots() {
        assert!(reject_route_path_segment("article", ".foo").is_ok());
        assert!(reject_route_path_segment("article", "..bar").is_ok());
    }

    #[test]
    fn rejects_the_current_directory_segment() {
        let error = reject_route_path_segment("article", ".")
            .expect_err("a single-dot segment is rejected");

        assert!(matches!(
            error,
            RouteUrlError::ReservedPathSegment { parameter, value }
                if parameter == "article" && value == "."
        ));
    }

    #[test]
    fn rejects_the_parent_directory_segment() {
        assert!(matches!(
            reject_route_path_segment("article", "..")
                .expect_err("a parent-dot segment is rejected"),
            RouteUrlError::ReservedPathSegment { value, .. } if value == ".."
        ));
    }

    #[test]
    fn names_the_parameter_and_value_in_its_message() {
        let message = reject_route_path_segment("article", "..")
            .expect_err("a parent-dot segment is rejected")
            .to_string();

        assert_eq!(
            message,
            "route parameter 'article' has the reserved path segment value '..'; `.` and `..` cannot form a URL path segment"
        );
    }
}
