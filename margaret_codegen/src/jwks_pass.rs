use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::render_jwks::render_jwks;

use crate::build_context::BuildContext;
use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_handler_path::public_jwks_handler_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;
use crate::jwks_verifier_path::public_jwks_verifier_canonical_path;

pub(crate) fn jwks_pass(context: &mut BuildContext, bindings: &ContainerBindings) {
    let has_roller = bindings.provides(&jwks_roller_canonical_path());
    let has_handler = bindings.provides(&public_jwks_handler_canonical_path());
    let has_client = bindings.provides(&jwks_client_canonical_path());
    let has_verifier = bindings.provides(&public_jwks_verifier_canonical_path());

    context.set_framework_services(jwks_framework_services(bindings));

    if has_roller || has_handler || has_client || has_verifier {
        context.extend_modules(vec![render_jwks(
            has_roller,
            has_handler,
            has_client,
            has_verifier,
        )]);
        context.enable_jwks();
    }
}
