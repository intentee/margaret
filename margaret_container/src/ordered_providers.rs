use margaret_attributes::canonical_path::CanonicalPath;

use crate::container_plan::ContainerPlan;
use crate::provider::Provider;

pub(crate) fn ordered_providers(plan: &ContainerPlan) -> Vec<(&CanonicalPath, &Provider)> {
    let mut ordered: Vec<(&CanonicalPath, &Provider)> = plan
        .providers
        .iter()
        .chain(plan.constructions.iter())
        .collect();

    ordered.sort_by(|(_, first), (_, second)| first.field_name.cmp(&second.field_name));

    ordered
}
