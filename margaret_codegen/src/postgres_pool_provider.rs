use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

use crate::postgres_pool_path::postgres_pool_canonical_path;

fn postgres_connection_uri_type() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "postgres_pool".to_string(),
        "postgres_connection_uri".to_string(),
        "PostgresConnectionUri".to_string(),
    ])
}

fn required_named_argument(name: &str, value_type: CanonicalPath) -> FrameworkDependency {
    FrameworkDependency::ConsoleArgument(Box::new(ConsoleArgument::Named {
        name: name.to_string(),
        required: true,
        weaving: WeavingKind::from_canonical(&value_type, true),
        value_type,
    }))
}

pub(crate) fn postgres_pool_provider(enablement: FrameworkEnablement) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                required_named_argument("postgres-url", postgres_connection_uri_type()),
                required_named_argument(
                    "postgres-max-connections",
                    CanonicalPath::new(vec!["u32".to_string()]),
                ),
            ],
            is_async: false,
            method: "connect".to_string(),
        },
        enablement,
        injection: FrameworkInjectionRole::Unmarked,
        provided: postgres_pool_canonical_path(),
    }
}
