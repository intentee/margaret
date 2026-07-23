use margaret_attributes::canonical_path::CanonicalPath;

const COPY_PRIMITIVES: [&str; 16] = [
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32",
    "u64", "u128", "usize",
];

fn is_copy_primitive(canonical: &CanonicalPath) -> bool {
    matches!(canonical.segments(), [name] if COPY_PRIMITIVES.contains(&name.as_str()))
}

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
pub enum ThreadingKind {
    Copy,
    BorrowedStr,
    BorrowedPath,
    Cloned,
}

impl ThreadingKind {
    #[must_use]
    pub fn from_canonical(canonical: &CanonicalPath, required: bool) -> Self {
        if is_copy_primitive(canonical) {
            return ThreadingKind::Copy;
        }

        if required && is_string(canonical) {
            return ThreadingKind::BorrowedStr;
        }

        if required && is_path_buf(canonical) {
            return ThreadingKind::BorrowedPath;
        }

        ThreadingKind::Cloned
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::ThreadingKind;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    #[test]
    fn a_required_copy_primitive_is_copy() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["u16"]), true),
            ThreadingKind::Copy
        );
    }

    #[test]
    fn an_optional_copy_primitive_is_still_copy() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["u16"]), false),
            ThreadingKind::Copy
        );
    }

    #[test]
    fn a_required_string_borrows_as_str() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["std", "string", "String"]), true),
            ThreadingKind::BorrowedStr
        );
    }

    #[test]
    fn an_optional_string_is_cloned() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["std", "string", "String"]), false),
            ThreadingKind::Cloned
        );
    }

    #[test]
    fn a_required_path_buf_borrows_as_path() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["std", "path", "PathBuf"]), true),
            ThreadingKind::BorrowedPath
        );
    }

    #[test]
    fn a_single_segment_non_primitive_is_cloned() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["Widget"]), true),
            ThreadingKind::Cloned
        );
    }

    #[test]
    fn a_resolved_custom_type_is_cloned() {
        assert_eq!(
            ThreadingKind::from_canonical(&path(&["crate", "Marker"]), true),
            ThreadingKind::Cloned
        );
    }
}
