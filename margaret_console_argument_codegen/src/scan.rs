use std::collections::BTreeMap;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::marker::marker;
use margaret_injection_codegen::parameters::parameters;

use crate::classify::classify;
use crate::console_argument::ConsoleArgument;
use crate::console_argument_arguments::ConsoleArgumentArguments;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_registry::ConsoleArgumentRegistry;

fn method_has_attribute(method: &IndexedMethod, name: &str) -> bool {
    let selector = AttributeSelector::from_marker(name);

    method
        .attributes()
        .iter()
        .any(|attribute| selector.matches(attribute.path()))
}

fn item_has_attribute(item: &IndexedItem, name: &str) -> bool {
    let selector = AttributeSelector::from_marker(name);

    item.attributes()
        .iter()
        .any(|attribute| selector.matches(attribute.path()))
}

fn single_constructor(item: &IndexedItem) -> Option<&IndexedMethod> {
    let mut constructors = item
        .methods()
        .iter()
        .filter(|method| method_has_attribute(method, "constructor"));

    let constructor = constructors.next()?;

    match constructors.next() {
        Some(_) => None,
        None => Some(constructor),
    }
}

fn validate_named_type_consistency(
    arguments: &BTreeMap<CanonicalPath, BTreeMap<usize, ConsoleArgument>>,
) -> Result<(), ConsoleArgumentCodegenError> {
    let mut declared: BTreeMap<String, (CanonicalPath, ConsoleArgument)> = BTreeMap::new();

    for (owner, by_position) in arguments {
        for argument in by_position.values() {
            if matches!(argument, ConsoleArgument::Positional { .. }) {
                continue;
            }

            let name = argument.name().to_string();

            match declared.get(&name) {
                Some((first_owner, first)) if first != argument => {
                    return Err(
                        ConsoleArgumentCodegenError::ConflictingConsoleArgumentType {
                            first_owner: first_owner.to_string(),
                            name,
                            owner: owner.to_string(),
                        },
                    );
                }
                Some(_) => {}
                None => {
                    declared.insert(name, (owner.clone(), argument.clone()));
                }
            }
        }
    }

    Ok(())
}

pub fn scan(
    index: &AttributeIndex,
) -> Result<ConsoleArgumentRegistry, ConsoleArgumentCodegenError> {
    let console_argument_selector = AttributeSelector::from_marker("console_argument");
    let mut arguments: BTreeMap<CanonicalPath, BTreeMap<usize, ConsoleArgument>> = BTreeMap::new();

    for item in index.items() {
        let Some(constructor) = single_constructor(item) else {
            continue;
        };

        let owner = item.canonical_path();
        let owner_text = owner.to_string();
        let is_command = item_has_attribute(item, "console_command");

        for view in parameters(constructor.signature()) {
            let Some(attribute) = marker(view.attributes, &console_argument_selector) else {
                continue;
            };

            let parameter = view.holder.to_string();
            let attribute_arguments = AttributeArgs::from_attribute(attribute)?;
            let ConsoleArgumentArguments { form, relations } =
                ConsoleArgumentArguments::parse(&attribute_arguments, &owner_text, &parameter)?;
            let argument = classify(
                index,
                item,
                form,
                &parameter,
                view.declared,
                &owner_text,
                relations,
            )?;

            if matches!(argument, ConsoleArgument::Positional { .. }) && !is_command {
                return Err(ConsoleArgumentCodegenError::PositionalOutsideCommand {
                    owner: owner_text,
                    parameter,
                });
            }

            arguments
                .entry(owner.clone())
                .or_default()
                .insert(view.position, argument);
        }
    }

    validate_named_type_consistency(&arguments)?;

    Ok(ConsoleArgumentRegistry::new(arguments))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;

    use super::scan;
    use crate::argument_relation::ArgumentRelation;
    use crate::console_argument::ConsoleArgument;
    use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
    use crate::console_argument_registry::ConsoleArgumentRegistry;

    fn registry_for(
        lib_source: &str,
    ) -> Result<ConsoleArgumentRegistry, ConsoleArgumentCodegenError> {
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

    fn describe(argument: &ConsoleArgument) -> String {
        match argument {
            ConsoleArgument::Flag { name } => format!("flag:{name}"),
            ConsoleArgument::Named {
                name,
                required,
                value_type,
                ..
            } => format!("named:{name}:{}:{value_type}", requiredness(*required)),
            ConsoleArgument::Positional {
                id,
                required,
                value_type,
                ..
            } => format!("positional:{id}:{}:{value_type}", requiredness(*required)),
        }
    }

    fn requiredness(required: bool) -> &'static str {
        if required { "required" } else { "optional" }
    }

    fn described(lib_source: &str, name: &str, position: usize) -> String {
        let registry = registry_for(lib_source).expect("the crate scans");

        describe(
            registry
                .argument(&owner(name), position)
                .expect("the argument is scanned"),
        )
    }

    #[test]
    fn scans_a_required_named_argument() {
        assert_eq!(
            described(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"label\")] label: String) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "named:label:required:std::string::String"
        );
    }

    #[test]
    fn scans_an_optional_named_argument() {
        assert_eq!(
            described(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"label\")] label: Option<String>) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "named:label:optional:std::string::String"
        );
    }

    #[test]
    fn scans_a_boolean_flag() {
        assert_eq!(
            described(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"loud\")] loud: bool) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "flag:loud"
        );
    }

    #[test]
    fn scans_a_required_positional_argument_on_a_command() {
        assert_eq!(
            described(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(positional)] name: String) -> Self {}\n}\n",
                "Greet",
                0,
            ),
            "positional:name:required:std::string::String"
        );
    }

    #[test]
    fn scans_an_optional_positional_argument() {
        assert_eq!(
            described(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(positional)] name: Option<String>) -> Self {}\n}\n",
                "Greet",
                0,
            ),
            "positional:name:optional:std::string::String"
        );
    }

    #[test]
    fn ignores_a_plain_dependency_parameter() {
        let registry = registry_for(
            "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(dependency: std::sync::Arc<Other>) -> Self {}\n}\n",
        )
        .expect("the crate scans");

        assert!(registry.argument(&owner("Config"), 0).is_none());
    }

    #[test]
    fn ignores_an_item_without_a_constructor() {
        let registry = registry_for("#[singleton]\nstruct Config;\n").expect("the crate scans");

        assert!(registry.argument(&owner("Config"), 0).is_none());
    }

    #[test]
    fn ignores_an_item_with_more_than_one_constructor() {
        let registry = registry_for(
            "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"a\")] a: String) -> Self {}\n\n    #[constructor]\n    fn make(#[console_argument(from = \"b\")] b: String) -> Self {}\n}\n",
        )
        .expect("the crate scans");

        assert!(registry.argument(&owner("Config"), 0).is_none());
    }

    fn error_message(lib_source: &str) -> String {
        registry_for(lib_source)
            .expect_err("the crate is rejected")
            .to_string()
    }

    #[test]
    fn rejects_a_positional_outside_a_command() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(positional)] name: String) -> Self {}\n}\n",
            )
            .contains("positional arguments are only allowed on console commands")
        );
    }

    #[test]
    fn rejects_a_boolean_positional() {
        assert!(
            error_message(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(positional)] loud: bool) -> Self {}\n}\n",
            )
            .contains("a boolean is a named flag, never positional")
        );
    }

    #[test]
    fn rejects_both_from_and_positional() {
        assert!(
            error_message(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(from = \"name\", positional)] name: String) -> Self {}\n}\n",
            )
            .contains("either named or positional, never both")
        );
    }

    #[test]
    fn rejects_neither_from_nor_positional() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument] name: String) -> Self {}\n}\n",
            )
            .contains("it must be exactly one of them")
        );
    }

    #[test]
    fn allows_a_shared_named_key_with_the_same_type() {
        let registry = registry_for(
            "#[singleton]\nstruct First;\n\nimpl First {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\")] path: String) -> Self {}\n}\n\n#[singleton]\nstruct Second;\n\nimpl Second {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\")] path: String) -> Self {}\n}\n",
        )
        .expect("a shared key with the same type is allowed");

        assert!(registry.argument(&owner("First"), 0).is_some());
        assert!(registry.argument(&owner("Second"), 0).is_some());
    }

    #[test]
    fn rejects_a_shared_named_key_with_a_different_type() {
        assert!(
            error_message(
                "#[singleton]\nstruct First;\n\nimpl First {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\")] path: String) -> Self {}\n}\n\n#[singleton]\nstruct Second;\n\nimpl Second {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\")] path: std::path::PathBuf) -> Self {}\n}\n",
            )
            .contains("must have the same type everywhere")
        );
    }

    #[test]
    fn reports_a_non_string_from_value() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = 5)] value: String) -> Self {}\n}\n",
            )
            .contains("failed to index the crate")
        );
    }

    #[test]
    fn scans_a_numeric_argument() {
        assert_eq!(
            described(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"count\")] count: u16) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "named:count:required:u16"
        );
    }

    #[test]
    fn scans_a_qualified_path_buf_argument() {
        assert_eq!(
            described(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"config\")] config: std::path::PathBuf) -> Self {}\n}\n",
                "Config",
                0,
            ),
            "named:config:required:std::path::PathBuf"
        );
    }

    #[test]
    fn rejects_a_generic_value_type() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"tags\")] tags: Vec<String>) -> Self {}\n}\n",
            )
            .contains("a console argument value type must be a single concrete type")
        );
    }

    #[test]
    fn rejects_an_unresolvable_value_type() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"widget\")] widget: Widget) -> Self {}\n}\n",
            )
            .contains("could not be resolved to a concrete type")
        );
    }

    #[test]
    fn rejects_a_non_path_value_type() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"pair\")] pair: (u8, u8)) -> Self {}\n}\n",
            )
            .contains("could not be resolved to a concrete type")
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

    #[test]
    fn scans_a_required_if_relation_on_a_named_argument() {
        let registry = registry_for(
            "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\", required_if = \"storage\", equals = \"file\")] path: Option<String>) -> Self {}\n}\n",
        )
        .expect("the crate scans");

        assert!(matches!(
            registry
                .argument(&owner("Config"), 0)
                .expect("the argument is scanned"),
            ConsoleArgument::Named { relations, .. }
                if relations
                    == &vec![ArgumentRelation::RequiredIfEq {
                        argument: "storage".to_string(),
                        value: "file".to_string(),
                    }]
        ));
    }

    #[test]
    fn rejects_a_required_if_without_an_equals_value() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\", required_if = \"storage\")] path: Option<String>) -> Self {}\n}\n",
            )
            .contains("incomplete `required_if` relation")
        );
    }

    #[test]
    fn rejects_an_equals_without_a_required_if_argument() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\", equals = \"file\")] path: Option<String>) -> Self {}\n}\n",
            )
            .contains("incomplete `required_if` relation")
        );
    }

    #[test]
    fn rejects_a_required_if_relation_on_a_flag() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"loud\", required_if = \"storage\", equals = \"file\")] loud: bool) -> Self {}\n}\n",
            )
            .contains("only supported on named value arguments")
        );
    }

    #[test]
    fn rejects_a_required_if_relation_on_a_positional() {
        assert!(
            error_message(
                "#[singleton]\n#[console_command(name = \"greet\")]\nstruct Greet;\n\nimpl Greet {\n    #[constructor]\n    fn create(#[console_argument(positional, required_if = \"storage\", equals = \"file\")] name: String) -> Self {}\n}\n",
            )
            .contains("only supported on named value arguments")
        );
    }

    #[test]
    fn reports_a_non_string_required_if_value() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\", required_if = 5, equals = \"file\")] path: Option<String>) -> Self {}\n}\n",
            )
            .contains("failed to index the crate")
        );
    }

    #[test]
    fn reports_a_non_string_equals_value() {
        assert!(
            error_message(
                "#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create(#[console_argument(from = \"path\", required_if = \"storage\", equals = 5)] path: Option<String>) -> Self {}\n}\n",
            )
            .contains("failed to index the crate")
        );
    }
}
