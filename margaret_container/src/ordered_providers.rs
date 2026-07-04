use crate::container_plan::ContainerPlan;
use crate::provider::Provider;

pub(crate) fn ordered_providers(plan: &ContainerPlan) -> Vec<&Provider> {
    let mut ordered: Vec<&Provider> = plan.providers.iter().collect();

    ordered.sort_by(|first, second| first.field_name.cmp(&second.field_name));

    ordered
}
