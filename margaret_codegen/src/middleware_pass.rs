use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::middleware_plans::middleware_plans;
use margaret_middleware_codegen::render_middleware_wrappers::render_middleware_wrappers;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn middleware_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    let plans = middleware_plans(context.index())?;
    let capabilities = context.capabilities();

    if capabilities.has_middleware && (capabilities.has_http || capabilities.has_websockets) {
        context.extend_modules(vec![GeneratedModuleTokens::new(
            "middleware",
            render_middleware_wrappers(&plans),
        )]);
    }

    context.set_middleware_plans(plans);

    Ok(())
}
