use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn model_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().has_models {
        return Ok(());
    }

    let module = margaret_model_codegen::render_models::render_models(context.index())?;

    context.extend_modules(vec![module]);

    Ok(())
}
