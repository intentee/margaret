use std::collections::BTreeSet;

use margaret_oauth_vocabulary::scope::Scope;

use crate::form_parameter::FormParameter;
use crate::target_audience::TargetAudience;

const SCOPE_SEPARATOR: &str = " ";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TokenTarget {
    pub audience: TargetAudience,
    pub scopes: BTreeSet<Scope>,
}

impl TokenTarget {
    #[must_use]
    pub fn form_parameters(&self) -> Vec<FormParameter> {
        let mut form = match &self.audience {
            TargetAudience::Audience(audience) => vec![FormParameter {
                name: "audience",
                value: audience.clone(),
            }],
            TargetAudience::Resource(resource) => vec![FormParameter {
                name: "resource",
                value: resource.to_string(),
            }],
            TargetAudience::Unspecified => Vec::new(),
        };

        if !self.scopes.is_empty() {
            form.push(FormParameter {
                name: "scope",
                value: self
                    .scopes
                    .iter()
                    .map(Scope::as_str)
                    .collect::<Vec<&str>>()
                    .join(SCOPE_SEPARATOR),
            });
        }

        form
    }
}
