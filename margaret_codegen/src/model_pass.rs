use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn model_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().has_models {
        return Ok(());
    }

    let models = margaret_model_codegen::models::models(context.index())?;
    let module = margaret_schema_codegen::render_schema::render_schema(&models);

    context.extend_modules(vec![module]);

    Ok(())
}
