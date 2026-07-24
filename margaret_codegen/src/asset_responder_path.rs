use margaret_asset_bag_codegen::asset_responder_identity::asset_responder_canonical_suffix;
use margaret_attributes::canonical_path::CanonicalPath;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn asset_responder_canonical_path() -> CanonicalPath {
    let mut segments = vec!["crate".to_string(), UMBRELLA_MODULE_NAME.to_string()];

    segments.extend(
        asset_responder_canonical_suffix()
            .iter()
            .map(|segment| (*segment).to_string()),
    );

    CanonicalPath::new(segments)
}
