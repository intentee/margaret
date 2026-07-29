use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_provider::AuthenticatedUserProvider;
use crate::authenticated_user_providers::authenticated_user_providers;
use crate::request_binding_error::RequestBindingError;
use crate::route_parameter_resolution::RouteParameterResolution;
use crate::route_parameter_resolutions::route_parameter_resolutions;
use crate::views_availability::ViewsAvailability;

pub struct BindingRegistries {
    pub authenticated_users: HashMap<CanonicalPath, AuthenticatedUserProvider>,
    pub route_parameters: HashMap<CanonicalPath, RouteParameterResolution>,
    pub views: ViewsAvailability,
}

impl BindingRegistries {
    /// # Errors
    ///
    /// Returns `RequestBindingError` propagated from the work it performs.
    pub fn collect(
        index: &AttributeIndex,
        views: ViewsAvailability,
    ) -> Result<Self, RequestBindingError> {
        let mut registries = Self {
            authenticated_users: HashMap::new(),
            route_parameters: route_parameter_resolutions(index)?,
            views,
        };

        registries.authenticated_users = authenticated_user_providers(index, &registries)?;

        Ok(registries)
    }

    #[must_use]
    pub fn providers(&self) -> Vec<&AuthenticatedUserProvider> {
        let mut providers: Vec<&AuthenticatedUserProvider> =
            self.authenticated_users.values().collect();

        providers
            .sort_by(|first, second| first.application.concrete.cmp(&second.application.concrete));

        providers
    }
}
