use margaret_attributes::canonical_path::CanonicalPath;

pub enum RequestInjectable {
    AssetBag,
    CurrentRequest,
    IntrospectedToken,
    Next,
    PeerSpiffeId,
    RequestBodyStream,
    Routes,
    UploadedFiles,
    ValidationResult,
    VerifiedJwt,
    Views,
}

impl RequestInjectable {
    #[must_use]
    pub fn resolve(resolved: Option<&CanonicalPath>, is_reference: bool) -> Option<Self> {
        [
            Self::AssetBag,
            Self::CurrentRequest,
            Self::IntrospectedToken,
            Self::Next,
            Self::PeerSpiffeId,
            Self::RequestBodyStream,
            Self::Routes,
            Self::UploadedFiles,
            Self::ValidationResult,
            Self::VerifiedJwt,
            Self::Views,
        ]
        .into_iter()
        .find(|injectable| injectable.matches(resolved, is_reference))
    }

    #[must_use]
    pub fn matches(&self, resolved: Option<&CanonicalPath>, is_reference: bool) -> bool {
        resolved == Some(&self.canonical_path()) && is_reference == self.requires_reference()
    }

    pub(crate) fn canonical_path(&self) -> CanonicalPath {
        match self {
            Self::AssetBag => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "asset_bag".to_string(),
                "asset_bag".to_string(),
                "AssetBag".to_string(),
            ]),
            Self::CurrentRequest => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "http".to_string(),
                "request".to_string(),
                "Request".to_string(),
            ]),
            Self::IntrospectedToken => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "token_introspection".to_string(),
                "introspected_token".to_string(),
                "IntrospectedToken".to_string(),
            ]),
            Self::Next => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "http".to_string(),
                "next".to_string(),
                "Next".to_string(),
            ]),
            Self::PeerSpiffeId => CanonicalPath::new(vec![
                "spiffe".to_string(),
                "spiffe_id".to_string(),
                "SpiffeId".to_string(),
            ]),
            Self::RequestBodyStream => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "http".to_string(),
                "request_body_stream".to_string(),
                "RequestBodyStream".to_string(),
            ]),
            Self::Routes => CanonicalPath::new(vec![
                "crate".to_string(),
                "margaret".to_string(),
                "routes".to_string(),
                "Routes".to_string(),
            ]),
            Self::UploadedFiles => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "http_uploaded_file".to_string(),
                "uploaded_files".to_string(),
                "UploadedFiles".to_string(),
            ]),
            Self::ValidationResult => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "validation".to_string(),
                "validation_result".to_string(),
                "ValidationResult".to_string(),
            ]),
            Self::VerifiedJwt => CanonicalPath::new(vec![
                "margaret".to_string(),
                "framework".to_string(),
                "jwt_verification".to_string(),
                "verified_jwt".to_string(),
                "VerifiedJwt".to_string(),
            ]),
            Self::Views => CanonicalPath::new(vec![
                "crate".to_string(),
                "margaret".to_string(),
                "views".to_string(),
                "Views".to_string(),
            ]),
        }
    }

    fn requires_reference(&self) -> bool {
        match self {
            Self::CurrentRequest | Self::PeerSpiffeId | Self::Routes | Self::Views => true,
            Self::AssetBag
            | Self::IntrospectedToken
            | Self::Next
            | Self::RequestBodyStream
            | Self::UploadedFiles
            | Self::ValidationResult
            | Self::VerifiedJwt => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::RequestInjectable;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    #[test]
    fn matches_a_reference_injectable_only_when_written_as_a_reference() {
        let routes = path(&["crate", "margaret", "routes", "Routes"]);

        assert!(RequestInjectable::Routes.matches(Some(&routes), true));
        assert!(!RequestInjectable::Routes.matches(Some(&routes), false));
    }

    #[test]
    fn matches_a_value_injectable_only_when_written_by_value() {
        let next = path(&["margaret", "framework", "http", "next", "Next"]);

        assert!(RequestInjectable::Next.matches(Some(&next), false));
        assert!(!RequestInjectable::Next.matches(Some(&next), true));
    }

    #[test]
    fn matches_the_asset_bag_injectable_only_when_written_by_value() {
        let asset_bag = path(&[
            "margaret",
            "framework",
            "asset_bag",
            "asset_bag",
            "AssetBag",
        ]);

        assert!(RequestInjectable::AssetBag.matches(Some(&asset_bag), false));
        assert!(!RequestInjectable::AssetBag.matches(Some(&asset_bag), true));
    }

    #[test]
    fn matches_the_views_injectable_only_when_written_as_a_reference() {
        let views = path(&["crate", "margaret", "views", "Views"]);

        assert!(RequestInjectable::Views.matches(Some(&views), true));
        assert!(!RequestInjectable::Views.matches(Some(&views), false));
    }

    #[test]
    fn matches_the_validation_result_injectable_only_when_written_by_value() {
        let validation_result = path(&[
            "margaret",
            "framework",
            "validation",
            "validation_result",
            "ValidationResult",
        ]);

        assert!(RequestInjectable::ValidationResult.matches(Some(&validation_result), false));
        assert!(!RequestInjectable::ValidationResult.matches(Some(&validation_result), true));
    }

    #[test]
    fn matches_the_current_request_and_peer_spiffe_id_only_as_references() {
        let request = path(&["margaret", "framework", "http", "request", "Request"]);
        let peer = path(&["spiffe", "spiffe_id", "SpiffeId"]);

        assert!(RequestInjectable::CurrentRequest.matches(Some(&request), true));
        assert!(!RequestInjectable::CurrentRequest.matches(Some(&request), false));
        assert!(RequestInjectable::PeerSpiffeId.matches(Some(&peer), true));
        assert!(!RequestInjectable::PeerSpiffeId.matches(Some(&peer), false));
    }

    #[test]
    fn matches_the_request_body_stream_only_when_written_by_value() {
        let stream = path(&[
            "margaret",
            "framework",
            "http",
            "request_body_stream",
            "RequestBodyStream",
        ]);

        assert!(RequestInjectable::RequestBodyStream.matches(Some(&stream), false));
        assert!(!RequestInjectable::RequestBodyStream.matches(Some(&stream), true));
    }

    #[test]
    fn matches_the_uploaded_files_only_when_written_by_value() {
        let files = path(&[
            "margaret",
            "framework",
            "http_uploaded_file",
            "uploaded_files",
            "UploadedFiles",
        ]);

        assert!(RequestInjectable::UploadedFiles.matches(Some(&files), false));
        assert!(!RequestInjectable::UploadedFiles.matches(Some(&files), true));
    }

    #[test]
    fn rejects_a_foreign_path() {
        let shadowed = path(&["crate", "app", "Routes"]);

        assert!(!RequestInjectable::Routes.matches(Some(&shadowed), true));
    }

    #[test]
    fn rejects_an_unresolved_type() {
        assert!(!RequestInjectable::CurrentRequest.matches(None, true));
    }
}
