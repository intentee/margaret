use margaret_attributes::canonical_path::CanonicalPath;

use crate::request_binding::RequestBinding;

#[must_use]
pub fn binding_root(binding: &RequestBinding) -> Option<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => Some(&application.concrete),
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => Some(binder_provider),
        RequestBinding::Injectable { dependency } => Some(&dependency.concrete),
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RequestBody
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => None,
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_container::injected_dependency::InjectedDependency;

    use super::binding_root;
    use crate::request_binding::RequestBinding;

    #[test]
    fn identifies_a_route_parameter_binder_as_a_retained_root() {
        let binder_provider =
            CanonicalPath::new(vec!["crate".to_string(), "UserBinder".to_string()]);
        let binding = RequestBinding::BoundRouteParameter {
            binder_field: "user_binder".to_string(),
            binder_provider: binder_provider.clone(),
            path_key: "user".to_string(),
        };

        assert_eq!(binding_root(&binding), Some(&binder_provider));
    }

    #[test]
    fn identifies_an_injected_dependency_as_a_retained_root() {
        let concrete = CanonicalPath::new(vec!["crate".to_string(), "Store".to_string()]);
        let binding = RequestBinding::Injectable {
            dependency: InjectedDependency {
                concrete: concrete.clone(),
                field: "store".to_string(),
            },
        };

        assert_eq!(binding_root(&binding), Some(&concrete));
    }
}
