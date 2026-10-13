use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::environment_variable::EnvironmentVariable;

#[must_use]
pub fn environment_variable_read(
    EnvironmentVariable { name, value }: &EnvironmentVariable,
) -> TokenStream {
    let name = name.as_str();
    let value_type = path_tokens(&value.value_type);
    let reader = if value.required {
        quote! { margaret::framework::environment_variable::read_required::read_required }
    } else {
        quote! { margaret::framework::environment_variable::read_optional::read_optional }
    };

    quote! {
        #reader::<#value_type>(#name)
            .map_err(margaret::framework::console::report_failure::report_failure)
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::environment_variable::EnvironmentVariable;
    use crate::environment_variable_name::EnvironmentVariableName;

    use super::environment_variable_read;

    fn variable(name: &str, required: bool, segments: &[&str]) -> EnvironmentVariable {
        EnvironmentVariable {
            name: EnvironmentVariableName::new(name).expect("the name is usable"),
            value: InputValue {
                required,
                value_type: CanonicalPath::new(segments.iter().map(ToString::to_string).collect()),
                weaving: WeavingKind::Cloned,
            },
        }
    }

    fn collapsed(variable: &EnvironmentVariable) -> String {
        environment_variable_read(variable)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_required_variable_reads_through_the_required_reader() {
        assert!(collapsed(&variable(
            "DATABASE_URL",
            true,
            &["std", "string", "String"],
        ))
        .contains(
            "margaret::framework::environment_variable::read_required::read_required::<::std::string::String>(\"DATABASE_URL\")"
        ));
    }

    #[test]
    fn an_optional_variable_reads_through_the_optional_reader() {
        assert!(collapsed(&variable("WORKER_COUNT", false, &["u16"])).contains(
            "margaret::framework::environment_variable::read_optional::read_optional::<u16>(\"WORKER_COUNT\")"
        ));
    }

    #[test]
    fn reports_a_failed_read_through_the_framework_so_no_failure_branch_is_generated() {
        assert!(
            collapsed(&variable(
                "DATABASE_URL",
                true,
                &["std", "string", "String"]
            ))
            .ends_with(".map_err(margaret::framework::console::report_failure::report_failure)")
        );
    }
}
