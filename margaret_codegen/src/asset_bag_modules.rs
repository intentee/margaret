use std::path::Path;

use margaret_asset_bag_codegen::asset_bag_generation::AssetBagGeneration;
use margaret_asset_bag_codegen::render_asset_bag::render_asset_bag;
use margaret_asset_bag_codegen::served_assets::ServedAssets;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::asset_macro_canonical_path::asset_macro_canonical_path;
use crate::asset_responder_canonical_path::asset_responder_canonical_path;
use crate::assets_directory_name::ASSETS_DIRECTORY_NAME;
use crate::codegen_error::CodegenError;
use crate::walk_asset_directory::walk_asset_directory;

fn served_assets(assets_directory: &Path) -> Result<ServedAssets, CodegenError> {
    Ok(ServedAssets {
        assets_directory_name: ASSETS_DIRECTORY_NAME.to_string(),
        served_tails: walk_asset_directory(assets_directory)?,
    })
}

pub(crate) fn asset_bag_modules(
    index: &AttributeIndex,
    metafile_contents: Option<&str>,
    bindings: &ContainerBindings,
    assets_directory: &Path,
) -> Result<Vec<GeneratedModuleTokens>, CodegenError> {
    let imports_macro = index.is_imported(&asset_macro_canonical_path());
    let requests_responder = bindings.provides(&asset_responder_canonical_path());
    if !imports_macro && !requests_responder {
        return Ok(Vec::new());
    }

    let Some(metafile) = metafile_contents else {
        return Err(if requests_responder {
            CodegenError::AssetResponderWithoutMetafile
        } else {
            CodegenError::AssetMacroWithoutMetafile
        });
    };
    let generation = if requests_responder {
        let served = served_assets(assets_directory)?;

        if imports_macro {
            AssetBagGeneration::MacroAndResponder(served)
        } else {
            AssetBagGeneration::Responder(served)
        }
    } else {
        AssetBagGeneration::Macro
    };

    Ok(render_asset_bag(metafile, &generation)?)
}
