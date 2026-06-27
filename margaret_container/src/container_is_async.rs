use margaret_attributes::attribute_index::AttributeIndex;

use crate::build_plan::build_plan;
use crate::container_error::ContainerError;

pub fn container_is_async(index: &AttributeIndex) -> Result<bool, ContainerError> {
    let plan = build_plan(index)?;

    Ok(plan.providers.iter().any(|provider| provider.is_async()))
}
