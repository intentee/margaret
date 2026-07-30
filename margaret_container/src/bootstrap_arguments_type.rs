use heck::ToUpperCamelCase;
use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn bootstrap_arguments_type(function: &Ident) -> Ident {
    format_ident!("{}Arguments", function.to_string().to_upper_camel_case())
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use super::bootstrap_arguments_type;

    #[test]
    fn names_the_serve_arguments_after_its_bootstrap_function() {
        assert_eq!(
            bootstrap_arguments_type(&format_ident!("serve")).to_string(),
            "ServeArguments"
        );
    }

    #[test]
    fn names_a_root_builder_argument_struct_after_its_bootstrap_function() {
        assert_eq!(
            bootstrap_arguments_type(&format_ident!("construct_commands_greet_greet")).to_string(),
            "ConstructCommandsGreetGreetArguments"
        );
    }
}
