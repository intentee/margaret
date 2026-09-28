use crate::asset_bag_module_name::ASSET_BAG_MODULE_NAME;

const RESPONDER_MODULE: &str = "asset_responder";
const RESPONDER_TYPE: &str = "AssetResponder";

#[must_use]
pub fn asset_responder_canonical_suffix() -> [&'static str; 3] {
    [ASSET_BAG_MODULE_NAME, RESPONDER_MODULE, RESPONDER_TYPE]
}
