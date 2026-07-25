use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::scan::scan;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::render_container::render_container;
use margaret_container_tests::container_module_source::container_module_source;

fn console_argument_constructor_provider() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                FrameworkDependency::ConsoleArgument(Box::new(ConsoleArgument::Named {
                    name: "widget-endpoint".to_string(),
                    required: true,
                    weaving: WeavingKind::Cloned,
                    value_type: CanonicalPath::new(vec![
                        "crate".to_string(),
                        "WidgetEndpoint".to_string(),
                    ]),
                })),
                FrameworkDependency::ConsoleArgument(Box::new(ConsoleArgument::Named {
                    name: "widget-count".to_string(),
                    required: true,
                    weaving: WeavingKind::Copy,
                    value_type: CanonicalPath::new(vec!["u32".to_string()]),
                })),
            ],
            is_async: false,
            method: "assemble".to_string(),
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::Unmarked,
        provided: CanonicalPath::new(vec!["crate".to_string(), "Widget".to_string()]),
    }
}

fn rendered_container() -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fieldless");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    container_module_source(
        render_container(&index, &registry, &[console_argument_constructor_provider()])
            .expect("the console-argument framework provider renders")
            .modules,
    )
    .split_whitespace()
    .collect()
}

#[test]
fn materializes_the_provider_as_a_concrete_field() {
    assert!(
        rendered_container()
            .contains("widget:tokio::sync::OnceCell<std::sync::Arc<crate::Widget>>")
    );
}

#[test]
fn parameterizes_the_accessor_with_both_console_arguments() {
    let source = rendered_container();

    assert!(source.contains(":crate::WidgetEndpoint"));
    assert!(source.contains(":u32"));
}

#[test]
fn constructs_the_concrete_type_and_wraps_it_in_a_new_arc() {
    assert!(
        rendered_container().contains("std::sync::Arc::new(crate::Widget::assemble(")
    );
}
