use std::path::Path;

use margaret_asset_bag_codegen::asset_bag_generation::AssetBagGeneration;
use margaret_asset_bag_codegen::render_asset_bag::render_asset_bag;
use margaret_asset_bag_codegen::served_assets::ServedAssets;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_container::container_bindings::ContainerBindings;

use crate::asset_bag_artifacts::AssetBagArtifacts;
use crate::asset_bag_inputs::AssetBagInputs;
use crate::asset_macro_canonical_path::asset_macro_canonical_path;
use crate::asset_responder_canonical_path::asset_responder_canonical_path;
use crate::codegen_error::CodegenError;
use crate::constants::ASSETS_DIRECTORY_NAME;
use crate::walk_asset_directory::walk_asset_directory;

fn rendered_artifacts(
    metafile: &str,
    inputs: AssetBagInputs,
    generation: &AssetBagGeneration,
) -> Result<AssetBagArtifacts, CodegenError> {
    Ok(AssetBagArtifacts {
        inputs,
        modules: render_asset_bag(metafile, generation)?,
    })
}

fn served_assets(assets_directory: &Path) -> Result<ServedAssets, CodegenError> {
    Ok(ServedAssets {
        assets_directory_name: ASSETS_DIRECTORY_NAME.to_string(),
        served_tails: walk_asset_directory(assets_directory)?,
    })
}

pub(crate) fn build_asset_bag_artifacts(
    index: &AttributeIndex,
    metafile_contents: Option<&str>,
    bindings: &ContainerBindings,
    assets_directory: &Path,
) -> Result<AssetBagArtifacts, CodegenError> {
    let imports_macro = index.is_imported(&asset_macro_canonical_path());
    let requests_responder = bindings.provides(&asset_responder_canonical_path());
    if !imports_macro && !requests_responder {
        return Ok(AssetBagArtifacts {
            inputs: AssetBagInputs::Unread,
            modules: Vec::new(),
        });
    }

    let Some(metafile) = metafile_contents else {
        return Err(if requests_responder {
            CodegenError::AssetResponderWithoutMetafile
        } else {
            CodegenError::AssetMacroWithoutMetafile
        });
    };

    if requests_responder {
        let served = served_assets(assets_directory)?;

        rendered_artifacts(
            metafile,
            AssetBagInputs::MetafileAndAssetsDirectory,
            &if imports_macro {
                AssetBagGeneration::MacroAndResponder(served)
            } else {
                AssetBagGeneration::Responder(served)
            },
        )
    } else {
        rendered_artifacts(
            metafile,
            AssetBagInputs::Metafile,
            &AssetBagGeneration::Macro,
        )
    }
}
