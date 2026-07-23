use margaret_asset_bag_codegen::render_asset_bag::render_asset_bag;
use margaret_asset_bag_codegen::responder_generation::ResponderGeneration;
use margaret_container::container_bindings::ContainerBindings;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn asset_bag_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    embed_relative: &str,
) -> Result<(), CodegenError> {
    let requested = bindings.provides(&asset_responder_canonical_path());

    let Some(metafile_contents) = context.metafile_contents() else {
        if requested {
            return Err(CodegenError::AssetResponderWithoutMetafile);
        }

        return Ok(());
    };

    let responder = if requested {
        ResponderGeneration::Emit {
            embed_relative: embed_relative.to_string(),
        }
    } else {
        ResponderGeneration::Skip
    };

    let modules = render_asset_bag(metafile_contents, responder)?;

    context.extend_modules(modules);

    Ok(())
}
