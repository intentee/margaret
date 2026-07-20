use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum HttpInjectable {
    AssetBag,
    CurrentRequest,
    Next,
    PeerSpiffeId,
    Routes,
    ValidationResult,
    Views,
}

impl HttpInjectable {
    pub(crate) fn canonical_path(&self) -> CanonicalPath {
        match self {
            Self::AssetBag => CanonicalPath::new(vec![
                "margaret_asset_bag".to_string(),
                "asset_bag".to_string(),
                "AssetBag".to_string(),
            ]),
            Self::CurrentRequest => CanonicalPath::new(vec![
                "margaret_http".to_string(),
                "request".to_string(),
                "Request".to_string(),
            ]),
            Self::Next => CanonicalPath::new(vec![
                "margaret_http".to_string(),
                "next".to_string(),
                "Next".to_string(),
            ]),
            Self::PeerSpiffeId => CanonicalPath::new(vec![
                "spiffe".to_string(),
                "spiffe_id".to_string(),
                "SpiffeId".to_string(),
            ]),
            Self::Routes => CanonicalPath::new(vec![
                "crate".to_string(),
                "margaret".to_string(),
                "routes".to_string(),
                "Routes".to_string(),
            ]),
            Self::ValidationResult => CanonicalPath::new(vec![
                "margaret_validation".to_string(),
                "validation_result".to_string(),
                "ValidationResult".to_string(),
            ]),
            Self::Views => CanonicalPath::new(vec![
                "crate".to_string(),
                "margaret".to_string(),
                "views".to_string(),
                "Views".to_string(),
            ]),
        }
    }

    pub(crate) fn matches(&self, resolved: Option<&CanonicalPath>, is_reference: bool) -> bool {
        resolved == Some(&self.canonical_path()) && is_reference == self.requires_reference()
    }

    fn requires_reference(&self) -> bool {
        match self {
            Self::CurrentRequest | Self::PeerSpiffeId | Self::Routes | Self::Views => true,
            Self::AssetBag | Self::Next | Self::ValidationResult => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::HttpInjectable;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    #[test]
    fn matches_a_reference_injectable_only_when_written_as_a_reference() {
        let routes = path(&["crate", "margaret", "routes", "Routes"]);

        assert!(HttpInjectable::Routes.matches(Some(&routes), true));
        assert!(!HttpInjectable::Routes.matches(Some(&routes), false));
    }

    #[test]
    fn matches_a_value_injectable_only_when_written_by_value() {
        let next = path(&["margaret_http", "next", "Next"]);

        assert!(HttpInjectable::Next.matches(Some(&next), false));
        assert!(!HttpInjectable::Next.matches(Some(&next), true));
    }

    #[test]
    fn matches_the_asset_bag_injectable_only_when_written_by_value() {
        let asset_bag = path(&["margaret_asset_bag", "asset_bag", "AssetBag"]);

        assert!(HttpInjectable::AssetBag.matches(Some(&asset_bag), false));
        assert!(!HttpInjectable::AssetBag.matches(Some(&asset_bag), true));
    }

    #[test]
    fn matches_the_views_injectable_only_when_written_as_a_reference() {
        let views = path(&["crate", "margaret", "views", "Views"]);

        assert!(HttpInjectable::Views.matches(Some(&views), true));
        assert!(!HttpInjectable::Views.matches(Some(&views), false));
    }

    #[test]
    fn rejects_a_foreign_path() {
        let shadowed = path(&["crate", "app", "Routes"]);

        assert!(!HttpInjectable::Routes.matches(Some(&shadowed), true));
    }

    #[test]
    fn rejects_an_unresolved_type() {
        assert!(!HttpInjectable::CurrentRequest.matches(None, true));
    }
}
