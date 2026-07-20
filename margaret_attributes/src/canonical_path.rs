use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use heck::ToSnakeCase;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalPath {
    segments: Vec<String>,
}

impl CanonicalPath {
    #[must_use]
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    #[must_use]
    pub fn field_name(&self) -> String {
        self.segments
            .iter()
            .skip(1)
            .map(|segment| segment.to_snake_case())
            .collect::<Vec<String>>()
            .join("_")
    }

    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl Display for CanonicalPath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", self.segments.join("::"))
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalPath;

    #[test]
    fn field_name_joins_the_snake_cased_module_path() {
        let path = CanonicalPath::new(vec![
            "crate".to_string(),
            "routes".to_string(),
            "GetGreeting".to_string(),
        ]);

        assert_eq!(path.field_name(), "routes_get_greeting");
    }

    #[test]
    fn field_name_disambiguates_repeated_leaves_across_modules() {
        let routes = CanonicalPath::new(vec![
            "crate".to_string(),
            "routes".to_string(),
            "get_users".to_string(),
            "GetUsers".to_string(),
        ]);
        let commands = CanonicalPath::new(vec![
            "crate".to_string(),
            "commands".to_string(),
            "get_users".to_string(),
            "GetUsers".to_string(),
        ]);

        assert_ne!(routes.field_name(), commands.field_name());
        assert_eq!(routes.field_name(), "routes_get_users_get_users");
        assert_eq!(commands.field_name(), "commands_get_users_get_users");
    }
}
