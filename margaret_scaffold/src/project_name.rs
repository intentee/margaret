const BASE_CRATE_SUFFIX: &str = "_base";
const IDENTITY_CRATE_SUFFIX: &str = "_identity";

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ProjectName {
    base_crate: String,
    identity_crate: String,
    target_directory: String,
}

impl ProjectName {
    pub(crate) fn new(target_directory: String) -> Self {
        Self {
            base_crate: format!("{target_directory}{BASE_CRATE_SUFFIX}"),
            identity_crate: format!("{target_directory}{IDENTITY_CRATE_SUFFIX}"),
            target_directory,
        }
    }

    #[must_use]
    pub(crate) fn base_crate(&self) -> &str {
        &self.base_crate
    }

    #[must_use]
    pub(crate) fn identity_crate(&self) -> &str {
        &self.identity_crate
    }

    #[must_use]
    pub(crate) fn target_directory(&self) -> &str {
        &self.target_directory
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectName;

    #[test]
    fn derives_both_member_crate_names_from_the_target_directory() {
        let project_name = ProjectName::new("acme".to_string());

        assert_eq!(project_name.target_directory(), "acme");
        assert_eq!(project_name.base_crate(), "acme_base");
        assert_eq!(project_name.identity_crate(), "acme_identity");
    }
}
