use heck::ToSnakeCase;

use crate::canonical_path::CanonicalPath;

#[must_use]
pub fn field_base(path: &CanonicalPath) -> String {
    path.segments()
        .iter()
        .skip(1)
        .map(|segment| segment.to_snake_case())
        .collect::<Vec<String>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use crate::canonical_path::CanonicalPath;
    use crate::field_base::field_base;

    #[test]
    fn joins_the_snake_cased_module_path() {
        let path = CanonicalPath::new(vec![
            "crate".to_string(),
            "routes".to_string(),
            "GetGreeting".to_string(),
        ]);

        assert_eq!(field_base(&path), "routes_get_greeting");
    }

    #[test]
    fn disambiguates_repeated_leaves_across_modules() {
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

        assert_ne!(field_base(&routes), field_base(&commands));
        assert_eq!(field_base(&routes), "routes_get_users_get_users");
        assert_eq!(field_base(&commands), "commands_get_users_get_users");
    }
}
