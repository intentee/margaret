use margaret_container::container_bindings::ContainerBindings;
use margaret_postgres_pool_codegen::render_postgres_pool::render_postgres_pool;

use crate::build_context::BuildContext;
use crate::postgres_pool_path::postgres_pool_canonical_path;

pub(crate) fn postgres_pool_pass(context: &mut BuildContext, bindings: &ContainerBindings) {
    if !bindings.provides(&postgres_pool_canonical_path()) {
        return;
    }

    context.extend_modules(vec![render_postgres_pool()]);
    context.enable_postgres_pool();
}
