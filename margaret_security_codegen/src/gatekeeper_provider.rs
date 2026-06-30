use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::synthetic_provider::SyntheticProvider;

use crate::security_plan::SecurityPlan;

pub(crate) fn backend_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        "margaret".to_string(),
        "security".to_string(),
        "SecurityBackend".to_string(),
    ])
}

pub(crate) fn gatekeeper_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        "margaret".to_string(),
        "security".to_string(),
        "Gatekeeper".to_string(),
    ])
}

pub(crate) fn synthetic_providers(plan: &SecurityPlan) -> Vec<SyntheticProvider> {
    let mut backend_dependencies = vec![plan.store_path.clone()];

    for gate in &plan.crud_gates {
        backend_dependencies.push(gate.gate_path.clone());
    }

    for gate in &plan.site_gates {
        backend_dependencies.push(gate.gate_path.clone());
    }

    vec![
        SyntheticProvider {
            concrete_path: backend_path(),
            constructor: "new".to_string(),
            dependencies: backend_dependencies,
            field_name: "security_backend".to_string(),
        },
        SyntheticProvider {
            concrete_path: gatekeeper_path(),
            constructor: "new".to_string(),
            dependencies: vec![backend_path()],
            field_name: "gatekeeper".to_string(),
        },
    ]
}
