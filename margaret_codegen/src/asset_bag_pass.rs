use std::path::Path;

use margaret_asset_bag_codegen::render_asset_bag::render_asset_bag;
use margaret_asset_bag_codegen::responder_generation::ResponderGeneration;
use margaret_container::container_bindings::ContainerBindings;

use crate::asset_responder_path::asset_responder_canonical_path;
use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::generate::ASSETS_DIRECTORY_NAME;
use crate::walk_asset_directory::walk_asset_directory;

pub(crate) fn asset_bag_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    assets_directory: &Path,
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
            assets_directory_name: ASSETS_DIRECTORY_NAME.to_string(),
            embed_relative: embed_relative.to_string(),
            served_tails: walk_asset_directory(assets_directory)?,
        }
    } else {
        ResponderGeneration::Skip
    };

    let modules = render_asset_bag(metafile_contents, responder)?;

    context.extend_modules(modules);

    Ok(())
}
