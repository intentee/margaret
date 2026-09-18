use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use quote::IdentFragment;

use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::server_codegen_error::ServerCodegenError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ServerName {
    name: String,
}

impl ServerName {
    /// # Errors
    ///
    /// Returns `ServerCodegenError` when the name cannot address a server.
    pub fn parse(name: String) -> Result<Self, ServerCodegenError> {
        if is_snake_case_identifier(&name) {
            Ok(Self { name })
        } else {
            Err(ServerCodegenError::ServerNameNotSnakeCase { name })
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl Display for ServerName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        formatter.write_str(&self.name)
    }
}

impl IdentFragment for ServerName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        formatter.write_str(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use super::ServerName;

    #[test]
    fn accepts_a_snake_case_name() {
        assert_eq!(
            ServerName::parse("public_api".to_string())
                .expect("a snake_case name addresses a server")
                .as_str(),
            "public_api"
        );
    }

    #[test]
    fn rejects_a_name_that_is_not_an_identifier() {
        assert_eq!(
            ServerName::parse("public-api".to_string())
                .expect_err("a hyphenated name cannot address a server")
                .to_string(),
            "the server name 'public-api' must be a snake_case identifier usable as a `routes` accessor"
        );
    }

    #[test]
    fn spells_an_identifier_fragment_as_the_parsed_name() {
        assert_eq!(
            format_ident!(
                "server_{}",
                ServerName::parse("internal".to_string())
                    .expect("a snake_case name addresses a server")
            )
            .to_string(),
            "server_internal"
        );
    }

    #[test]
    fn displays_the_parsed_name() {
        assert_eq!(
            ServerName::parse("internal".to_string())
                .expect("a snake_case name addresses a server")
                .to_string(),
            "internal"
        );
    }
}
