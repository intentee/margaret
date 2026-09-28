use crate::asset_bag_module_name::ASSET_BAG_MODULE_NAME;

const ASSET_MACRO_NAME: &str = "asset";

#[must_use]
pub fn asset_macro_canonical_suffix() -> [&'static str; 2] {
    [ASSET_BAG_MODULE_NAME, ASSET_MACRO_NAME]
}
