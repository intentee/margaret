use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes_tests::indexed_source::IndexedSource;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

#[test]
fn hands_the_spiffe_client_to_a_framework_constructor() {
    let index = IndexedSource::new("").index;
    let provided = CanonicalPath::new(vec!["crate".to_string(), "Workload".to_string()]);
    let rendered = render_container(
        &index,
        &scan(&index).expect("the serve inputs are scanned"),
        &[FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::SpiffeHttpClient],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::Unmarked,
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
            .expect("the workload has planned serve inputs")
            .iter()
            .map(|slotted| slotted.input.name())
            .collect::<Vec<&str>>(),
        ["spiffe_http_client"]
    );
}
