use margaret_container::container_bindings::ContainerBindings;
use margaret_database_codegen::database_canonical_path::database_canonical_path;

use crate::route_parameter_binder::RouteParameterBinder;

pub enum RouteDatabase {
    Declared(RouteParameterBinder),
    Undeclared,
}

impl RouteDatabase {
    #[must_use]
    pub fn of(container_bindings: &ContainerBindings) -> Self {
        let database = database_canonical_path();

        match container_bindings.provider(&database) {
            Some(binding) => Self::Declared(RouteParameterBinder {
                field: binding.field_name.clone(),
                provider: database,
            }),
            None => Self::Undeclared,
        }
    }
}
