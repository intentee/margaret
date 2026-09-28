use margaret_asset_bag_codegen::asset_responder_canonical_suffix::asset_responder_canonical_suffix;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

pub(crate) fn asset_responder_canonical_path() -> CanonicalPath {
    umbrella_item_path(&asset_responder_canonical_suffix())
}
