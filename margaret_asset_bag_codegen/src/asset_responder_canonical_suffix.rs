const ASSET_BAG_MODULE: &str = "asset_bag";
const RESPONDER_MODULE: &str = "asset_responder";
const RESPONDER_TYPE: &str = "AssetResponder";

#[must_use]
pub fn asset_responder_canonical_suffix() -> [&'static str; 3] {
    [ASSET_BAG_MODULE, RESPONDER_MODULE, RESPONDER_TYPE]
}
