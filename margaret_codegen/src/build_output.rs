use crate::asset_bag_inputs::AssetBagInputs;
use crate::generated_code::GeneratedCode;

#[derive(Debug)]
pub struct BuildOutput {
    pub asset_bag_inputs: AssetBagInputs,
    pub code: GeneratedCode,
}
