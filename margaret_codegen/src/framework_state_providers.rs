use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_database_codegen::database_canonical_path::database_canonical_path;
use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
use margaret_database_codegen::postgres_database_declaration::PostgresDatabaseDeclaration;

use crate::database_url_canonical_path::database_url_canonical_path;
use crate::framework_state_store::FrameworkStateStore;
use crate::framework_state_stores::framework_state_stores;

pub(crate) fn framework_state_providers(
    database: &DeclaredPostgresDatabase,
) -> Vec<FrameworkProvider> {
    let stores =
        framework_state_stores()
            .into_iter()
            .map(|FrameworkStateStore { provided, .. }| FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::Database],
                    is_async: false,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Infallible,
                },
                enablement: FrameworkEnablement::Dependency,
                injection: FrameworkInjectionRole::FrameworkState,
                provided,
            });

    match database {
        DeclaredPostgresDatabase::Absent => stores.collect(),
        DeclaredPostgresDatabase::Declared(PostgresDatabaseDeclaration { url_from, .. }) => stores
            .chain([FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::EnvironmentVariable {
                        name: url_from.clone(),
                        value_type: database_url_canonical_path(),
                    }],
                    is_async: true,
                    method: "connect".to_string(),
                    outcome: ConstructorOutcome::Fallible,
                },
                enablement: FrameworkEnablement::Declared,
                injection: FrameworkInjectionRole::Unmarked,
                provided: database_canonical_path(),
            }])
            .collect(),
    }
}
