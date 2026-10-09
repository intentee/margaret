use margaret_attributes::canonical_path::CanonicalPath;

use crate::umbrella_item_path::umbrella_item_path;

#[must_use]
pub fn routes_path() -> CanonicalPath {
    umbrella_item_path(&["routes", "Routes"])
}

#[cfg(test)]
mod tests {
    use super::routes_path;

    #[test]
    fn locates_the_generated_routes_of_the_crate() {
        assert_eq!(routes_path().to_string(), "crate::margaret::routes::Routes");
    }
}
