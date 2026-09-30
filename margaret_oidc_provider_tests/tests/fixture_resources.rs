use std::collections::BTreeSet;

use margaret_registered_claims::audience::Audience;

/// # Panics
///
/// Panics when a fixture resource is not an audience.
#[must_use]
pub fn fixture_resources(resources: &[&str]) -> BTreeSet<Audience> {
    resources
        .iter()
        .map(|resource| {
            resource
                .parse()
                .expect("the fixture resource is an audience")
        })
        .collect()
}
