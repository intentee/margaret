use margaret_attributes::canonical_path::CanonicalPath;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

#[must_use]
pub fn umbrella_item_path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(
        ["crate", UMBRELLA_MODULE_NAME]
            .iter()
            .chain(segments)
            .map(|segment| (*segment).to_string())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::umbrella_item_path;

    #[test]
    fn roots_the_item_in_the_umbrella_module_of_the_crate() {
        assert_eq!(
            umbrella_item_path(&["jwks", "JwksRoller"]).to_string(),
            "crate::margaret::jwks::JwksRoller"
        );
    }
}
