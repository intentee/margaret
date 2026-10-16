use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::asset_bag_inputs::AssetBagInputs;

pub(crate) struct AssetBagArtifacts {
    pub(crate) inputs: AssetBagInputs,
    pub(crate) modules: Vec<GeneratedModuleTokens>,
}
