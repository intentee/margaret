use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::is_copy_primitive::is_copy_primitive;

fn is_string(canonical: &CanonicalPath) -> bool {
    canonical
        == &CanonicalPath::new(vec![
            "std".to_string(),
            "string".to_string(),
            "String".to_string(),
        ])
}

fn is_path_buf(canonical: &CanonicalPath) -> bool {
    canonical
        == &CanonicalPath::new(vec![
            "std".to_string(),
            "path".to_string(),
            "PathBuf".to_string(),
        ])
}

#[derive(Clone, Debug, PartialEq)]
pub enum WeavingKind {
    Copy,
    BorrowedStr,
    BorrowedPath,
    Cloned,
}

impl WeavingKind {
    #[must_use]
    pub fn from_canonical(canonical: &CanonicalPath, required: bool) -> Self {
        if matches!(canonical.segments(), [name] if is_copy_primitive(name)) {
            return WeavingKind::Copy;
        }

        if required && is_string(canonical) {
            return WeavingKind::BorrowedStr;
        }

        if required && is_path_buf(canonical) {
            return WeavingKind::BorrowedPath;
        }

        WeavingKind::Cloned
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::WeavingKind;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(
            segments
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }

    #[test]
    fn a_required_copy_primitive_is_copy() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["u16"]), true),
            WeavingKind::Copy
        );
    }

    #[test]
    fn an_optional_copy_primitive_is_still_copy() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["u16"]), false),
            WeavingKind::Copy
        );
    }

    #[test]
    fn a_required_string_borrows_as_str() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["std", "string", "String"]), true),
            WeavingKind::BorrowedStr
        );
    }

    #[test]
    fn an_optional_string_is_cloned() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["std", "string", "String"]), false),
            WeavingKind::Cloned
        );
    }

    #[test]
    fn a_required_path_buf_borrows_as_path() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["std", "path", "PathBuf"]), true),
            WeavingKind::BorrowedPath
        );
    }

    #[test]
    fn a_single_segment_non_primitive_is_cloned() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["Widget"]), true),
            WeavingKind::Cloned
        );
    }

    #[test]
    fn a_resolved_custom_type_is_cloned() {
        assert_eq!(
            WeavingKind::from_canonical(&path(&["crate", "Marker"]), true),
            WeavingKind::Cloned
        );
    }
}
