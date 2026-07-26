use margaret_request_binding_codegen::binding_registries::BindingRegistries;
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

    BindingRegistries::collect(context.index(), views).map_err(CodegenError::from)
}
