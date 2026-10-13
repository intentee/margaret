use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes_tests::indexed_source::IndexedSource;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_preparation::FrameworkPreparation;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::preparation_source::PreparationSource;
use margaret_container::render_container::render_container;
use margaret_container::rendered_container::RenderedContainer;
use margaret_container_tests::container_module_source::container_module_source;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_serve_input_codegen::scan::scan;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
}

fn render(provider: FrameworkProvider) -> RenderedContainer {
    let index = IndexedSource::new("").index;

    render_container(
        &index,
        &scan(&index).expect("the serve inputs are scanned"),
        &[provider],
        &DeclaredPostgresDatabase::Absent,
        &DeclaredTokenIssuance::Absent,
    )
    .expect("the container renders")
}

fn serve_input_names(rendered: &RenderedContainer, provided: CanonicalPath) -> Vec<String> {
    rendered
        .bindings
        .serve_inputs(&[provided])
        .expect("the provider has planned serve inputs")
        .iter()
        .map(|slotted| slotted.input.name())
        .collect()
}

#[test]
fn takes_a_whole_prepared_provider_from_the_parse_phase() {
    let rendered = render(FrameworkProvider {
        construction: FrameworkConstruction::Prepared {
            method: "of_provider".to_string(),
            sources: vec![PreparationSource::Constant(path(&["crate", "ENDPOINTS"]))],
        },
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: path(&["crate", "Metadata"]),
    });

    assert_eq!(
        serve_input_names(&rendered, path(&["crate", "Metadata"])),
        ["crate::Metadata::of_provider"]
    );

    let source: String = container_module_source(rendered.modules)
        .split_whitespace()
        .collect();

    assert!(source.contains("pubargument0:crate::Metadata,"));
    assert!(source.contains("letmetadata=::std::sync::Arc::new(serve_input_0);"));
}

#[test]
fn hands_a_prepared_argument_to_an_infallible_framework_constructor() {
    let rendered = render(FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![FrameworkDependency::Prepared(FrameworkPreparation {
                function: path(&["crate", "Grant", "target"]),
                prepared: path(&["crate", "Target"]),
                sources: vec![PreparationSource::Constant(path(&["crate", "GRANT"]))],
            })],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Declared,
        injection: FrameworkInjectionRole::Unmarked,
        provided: path(&["crate", "Credentials"]),
    });

    assert_eq!(
        serve_input_names(&rendered, path(&["crate", "Credentials"])),
        ["crate::Grant::target"]
    );

    let source: String = container_module_source(rendered.modules)
        .split_whitespace()
        .collect();

    assert!(source.contains("pubargument0:crate::Target,"));
    assert!(source.contains(
        "letcredentials=::std::sync::Arc::new(crate::Credentials::create(serve_input_0));"
    ));
}

#[test]
fn rejects_a_prepared_argument_from_a_token_issuance_without_a_declared_issuer() {
    let index = IndexedSource::new("").index;

    assert!(matches!(
        render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::Prepared(FrameworkPreparation {
                        function: path(&["crate", "Handler", "of_issuance"]),
                        prepared: path(&["crate", "Issuance"]),
                        sources: vec![PreparationSource::TokenIssuance],
                    })],
                    is_async: false,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Infallible,
                },
                enablement: FrameworkEnablement::Declared,
                injection: FrameworkInjectionRole::Unmarked,
                provided: path(&["crate", "Handler"]),
            }],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        ),
        Err(ContainerError::MissingTokenIssuance { provider }) if provider == "crate::Handler"
    ));
}
