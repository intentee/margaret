use margaret_accepted_clients::non_empty_set::NonEmptySet;
use margaret_registered_claims::audience::Audience;

fn audience(resource: &str) -> Audience {
    resource
        .parse()
        .expect("the fixture resource is an audience")
}

/// # Panics
///
/// Panics when a fixture resource is not an audience.
#[must_use]
pub fn fixture_resources(first: &str, others: &[&str]) -> NonEmptySet<Audience> {
    NonEmptySet::of(
        audience(first),
        others.iter().map(|resource| audience(resource)),
    )
}
