use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_console_argument_codegen::console_argument_scope::ConsoleArgumentScope;
use margaret_console_argument_codegen::read_console_argument::read_console_argument;
use margaret_environment_variable_codegen::read_environment_variable::read_environment_variable;
use margaret_injection_codegen::parameters::parameters;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;

use crate::declared_serve_inputs::DeclaredServeInputs;
use crate::serve_input::ServeInput;
use crate::serve_input_codegen_error::ServeInputCodegenError;
use crate::serve_input_source::DeclaredServeInputSource;
use crate::serve_input_source::serve_input_source;

fn console_argument_scope(item: &IndexedItem) -> ConsoleArgumentScope {
    if item.has_framework_attribute(FrameworkAttribute::ConsoleCommand) {
        ConsoleArgumentScope::ConsoleCommand
    } else {
        ConsoleArgumentScope::ServeConstruction
    }
}

fn single_constructor(item: &IndexedItem) -> Option<&IndexedMethod> {
    let mut constructors = item
        .methods()
        .iter()
        .filter(|method| method.has_framework_attribute(FrameworkAttribute::Constructor));

    let constructor = constructors.next()?;

    match constructors.next() {
        Some(_) => None,
        None => Some(constructor),
    }
}

fn scan_item(
    index: &AttributeIndex,
    item: &IndexedItem,
    constructor: &IndexedMethod,
) -> Result<BTreeMap<usize, ServeInput>, ServeInputCodegenError> {
    let owner = item.canonical_path();
    let scope = console_argument_scope(item);
    let mut inputs = BTreeMap::new();

    for view in parameters(constructor) {
        let site = ConstructorParameter {
            owner: owner.clone(),
            parameter: view.holder.to_string(),
        };
        let Some(DeclaredServeInputSource {
            attribute,
            declared_by,
        }) = serve_input_source(view.attributes, &site)?
        else {
            continue;
        };
        let arguments = attribute.args()?;

        let input = match declared_by {
            FrameworkAttribute::ConsoleArgument => ServeInput::ConsoleArgument(
                read_console_argument(index, item, arguments, view.declared, &site, scope)?,
            ),
            FrameworkAttribute::EnvironmentVariable => ServeInput::EnvironmentVariable(
                read_environment_variable(index, item, arguments, view.declared, &site)?,
            ),
            _ if attribute.is_bare() => ServeInput::SpiffeHttpClient,
            _ => {
                return Err(ServeInputCodegenError::SpiffeHttpClientTakesNoArguments { site });
            }
        };

        inputs.insert(view.position, input);
    }

    Ok(inputs)
}

/// # Errors
///
/// Returns `ServeInputCodegenError` propagated from the work it performs.
pub fn scan(index: &AttributeIndex) -> Result<DeclaredServeInputs, ServeInputCodegenError> {
    let mut inputs: BTreeMap<CanonicalPath, BTreeMap<usize, ServeInput>> = BTreeMap::new();

    for item in index.items() {
        let Some(constructor) = single_constructor(item) else {
            continue;
        };
        let scanned = scan_item(index, item, constructor)?;

        if !scanned.is_empty() {
            inputs.insert(item.canonical_path().clone(), scanned);
        }
    }

    Ok(DeclaredServeInputs::new(inputs))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;

    use crate::declared_serve_inputs::DeclaredServeInputs;
    use crate::serve_input::ServeInput;
    use crate::serve_input_codegen_error::ServeInputCodegenError;

    use super::scan;

    fn scanned(lib_source: &str) -> Result<DeclaredServeInputs, ServeInputCodegenError> {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        scan(
            &AttributeIndexBuilder::new()
                .index_crate(&CrateRoot::new("crate", source_directory))
                .expect("the crate is indexed")
                .build(),
        )
    }

    fn owner(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn describe(lib_source: &str, name: &str, position: usize) -> String {
        let declared = scanned(lib_source).expect("the crate scans");
        let input = declared
            .input(&owner(name), position)
            .expect("the input is scanned");

        match input {
            ServeInput::ConsoleArgument(ConsoleArgument::Flag { name }) => format!("flag:{name}"),
            ServeInput::ConsoleArgument(ConsoleArgument::Named { name, value }) => format!(
                "named:{name}:{}:{}",
                requiredness(value.required),
                value.value_type
            ),
            ServeInput::ConsoleArgument(ConsoleArgument::Positional { id, value }) => format!(
                "positional:{id}:{}:{}",
                requiredness(value.required),
                value.value_type
            ),
            ServeInput::EnvironmentVariable(variable) => format!(
                "environment:{}:{}:{}",
                variable.name,
                requiredness(variable.value.required),
                variable.value.value_type
            ),
            ServeInput::SpiffeHttpClient => "spiffe_http_client".to_string(),
        }
    }

    fn requiredness(required: bool) -> &'static str {
        if required { "required" } else { "optional" }
    }

    fn error_message(lib_source: &str) -> String {
        scanned(lib_source)
            .expect_err("the crate is rejected")
            .to_string()
    }

    #[test]
    fn scans_a_required_named_console_argument() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"label\")] label: String) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "named:label:required:std::string::String"
        );
    }

    #[test]
    fn scans_a_boolean_console_flag() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"loud\")] loud: bool) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "flag:loud"
        );
    }

    #[test]
    fn scans_a_positional_console_argument_on_a_command() {
        assert_eq!(
            describe(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(positional)] name: String) -> Self {}\n}\n",
                "Greet",
                0,
            ),
            "positional:name:required:std::string::String"
        );
    }

    #[test]
    fn scans_a_required_environment_variable() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[environment_variable(from = \"DATABASE_URL\")] url: String) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "environment:DATABASE_URL:required:std::string::String"
        );
    }

    #[test]
    fn scans_an_optional_environment_variable() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[environment_variable(from = \"WORKER_COUNT\")] workers: Option<u16>) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "environment:WORKER_COUNT:optional:u16"
        );
    }

    #[test]
    fn scans_a_boolean_environment_variable_as_a_parsed_value() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[environment_variable(from = \"WORKER_DEBUG\")] debug: bool) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "environment:WORKER_DEBUG:required:bool"
        );
    }

    #[test]
    fn scans_a_spiffe_http_client_injection() {
        assert_eq!(
            describe(
                "#[singleton]\nstruct Caller;\n\nimpl Caller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: reqwest::Client) -> Self {}\n}\n",
                "Caller",
                0,
            ),
            "spiffe_http_client"
        );
    }

    #[test]
    fn ignores_a_plain_dependency_parameter() {
        let declared = scanned(
            "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(dependency: std::sync::Arc<Other>) -> Self {}\n}\n",
        )
        .expect("the crate scans");

        assert!(declared.input(&owner("Config"), 0).is_none());
    }

    #[test]
    fn ignores_an_item_without_a_constructor() {
        let declared = scanned("#[singleton]\nstruct Config;\n").expect("the crate scans");

        assert!(declared.input(&owner("Config"), 0).is_none());
    }

    #[test]
    fn ignores_an_item_with_more_than_one_constructor() {
        let declared = scanned(
            "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"a\")] a: String) -> Self {}\n\n    #[constructor]\n    fn make(#[console_argument(from = \"b\")] b: String) -> Self {}\n}\n",
        )
        .expect("the crate scans");

        assert!(declared.input(&owner("Config"), 0).is_none());
    }

    #[test]
    fn rejects_a_parameter_that_declares_two_serve_inputs() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"label\")] #[environment_variable(from = \"LABEL\")] label: String) -> Self {}\n}\n",
            )
            .contains("a constructor parameter is fed by exactly one serve input")
        );
    }

    #[test]
    fn rejects_a_spiffe_http_client_that_carries_arguments() {
        assert!(
            error_message(
                "#[singleton]\nstruct Caller;\n\nimpl Caller {\n    #[constructor]\n    fn create(#[spiffe_http_client(pooled)] client: reqwest::Client) -> Self {}\n}\n",
            )
            .contains("the injected client takes no arguments")
        );
    }

    #[test]
    fn rejects_a_positional_console_argument_outside_a_command() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(positional)] name: String) -> Self {}\n}\n",
            )
            .contains("positional arguments are only allowed on console commands")
        );
    }

    #[test]
    fn rejects_an_environment_variable_with_a_malformed_name() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[environment_variable(from = \"DATABASE-URL\")] url: String) -> Self {}\n}\n",
            )
            .contains("is not a usable environment variable name")
        );
    }

    #[test]
    fn reports_unparseable_attribute_arguments() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(= 5)] value: String) -> Self {}\n}\n",
            )
            .contains("failed to index the crate")
        );
    }
}
