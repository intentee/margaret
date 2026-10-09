use serde::Deserialize;

use crate::module_result::ModuleResult;
use crate::module_status::ModuleStatus;

#[derive(Debug, Deserialize)]
pub struct ModuleInfo {
    pub result: ModuleResult,
    pub status: ModuleStatus,
}
