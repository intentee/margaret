use serde_json::Value;

use crate::module_info::ModuleInfo;

pub struct ModuleOutcome {
    pub info: ModuleInfo,
    pub log: Value,
}
