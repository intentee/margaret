use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
use margaret_request_binding_codegen::views_availability::ViewsAvailability;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn binding_pass(context: &mut BuildContext) -> Result<BindingRegistries, CodegenError> {
    let capabilities = context.capabilities();
    let views = if capabilities.has_views {
        ViewsAvailability::Available
    } else {
        ViewsAvailability::Unavailable
    };
    let registries = BindingRegistries::collect(context.index(), views)?;

    if capabilities.has_authenticated_users
        && (capabilities.has_http || capabilities.has_websockets)
    {
        context.extend_modules(vec![GeneratedModuleTokens::new(
            "authenticated_users",
            render_authenticated_user_wrappers(&registries.providers()),
        )]);
    }

    Ok(registries)
}
