use crate::served_assets::ServedAssets;

pub enum AssetBagGeneration {
    Macro,
    MacroAndResponder(ServedAssets),
    Responder(ServedAssets),
}
