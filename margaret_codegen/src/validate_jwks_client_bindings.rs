use std::collections::HashMap;

use margaret_jwks_codegen::jwks_client_module_segment::jwks_client_module_segment;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::codegen_error::CodegenError;

pub(crate) fn validate_jwks_client_bindings(
    client_bindings: &[JwksClientBinding],
) -> Result<(), CodegenError> {
    let mut modules: HashMap<String, String> = HashMap::new();

    for binding in client_bindings {
        let module = jwks_client_module_segment(&binding.tag);
        let tag = binding.tag.to_string();

        if let Some(first) = modules.insert(module.clone(), tag.clone()) {
            return Err(CodegenError::CollidingJwksClientModules {
                first,
                module,
                second: tag,
            });
        }
    }

    Ok(())
}
