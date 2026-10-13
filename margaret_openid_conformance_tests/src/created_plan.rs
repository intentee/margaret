use serde::Deserialize;

use crate::plan_module::PlanModule;

#[derive(Debug, Deserialize)]
pub struct CreatedPlan {
    pub id: String,
    pub modules: Vec<PlanModule>,
}
