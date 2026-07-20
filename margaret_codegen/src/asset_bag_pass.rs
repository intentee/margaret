use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn asset_bag_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    let Some(metafile_contents) = context.metafile_contents() else {
        return Ok(());
    };

    let modules =
        margaret_asset_bag_codegen::render_asset_bag::render_asset_bag(metafile_contents)?;

    context.extend_modules(modules);

    Ok(())
}
