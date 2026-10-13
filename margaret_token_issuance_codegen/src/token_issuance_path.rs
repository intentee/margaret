use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::token_issuance_module_name::TOKEN_ISSUANCE_MODULE_NAME;

#[must_use]
pub fn token_issuance_path() -> CanonicalPath {
    umbrella_item_path(&[TOKEN_ISSUANCE_MODULE_NAME, "TOKEN_ISSUANCE"])
}

#[cfg(test)]
mod tests {
    use super::token_issuance_path;

    #[test]
    fn roots_the_token_issuance_in_its_module() {
        assert_eq!(
            token_issuance_path().to_string(),
            "crate::margaret::token_issuance::TOKEN_ISSUANCE"
        );
    }
}
