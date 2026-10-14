use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes_tests::indexed_source::IndexedSource;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

#[test]
fn reads_a_framework_provider_straight_from_its_environment_variable() {
    let index = IndexedSource::new("").index;
    let provided = CanonicalPath::new(vec!["crate".to_string(), "SealingKey".to_string()]);
    let rendered = render_container(
        &index,
        &scan(&index).expect("the serve inputs are scanned"),
        &[FrameworkProvider {
            construction: FrameworkConstruction::EnvironmentVariable {
                name: EnvironmentVariableName::new("SEALING_KEY")
                    .expect("the name is an environment variable name"),
            },
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::FrameworkState,
            provided: provided.clone(),
        }],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::Absent,
    )
    .expect("the container renders");

    assert_eq!(
        rendered
            .bindings
            .serve_inputs(&[provided])
            .expect("the provider has planned serve inputs")
            .iter()
            .map(|slotted| slotted.input.name())
            .collect::<Vec<String>>(),
        ["SEALING_KEY"]
    );

    let source: String = container_module_source(rendered.modules)
        .split_whitespace()
        .collect();

    assert!(source.contains("pubargument0:crate::SealingKey,"));
    assert!(source.contains("letsealing_key=::std::sync::Arc::new(serve_input_0);"));
}
