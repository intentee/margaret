use crate::canonical_path::CanonicalPath;

const STANDARD_LIBRARY_COPY_TYPES: [[&str; 2]; 18] = [
    ["net", "IpAddr"],
    ["net", "Ipv4Addr"],
    ["net", "Ipv6Addr"],
    ["net", "SocketAddr"],
    ["net", "SocketAddrV4"],
    ["net", "SocketAddrV6"],
    ["num", "NonZeroI8"],
    ["num", "NonZeroI16"],
    ["num", "NonZeroI32"],
    ["num", "NonZeroI64"],
    ["num", "NonZeroI128"],
    ["num", "NonZeroIsize"],
    ["num", "NonZeroU8"],
    ["num", "NonZeroU16"],
    ["num", "NonZeroU32"],
    ["num", "NonZeroU64"],
    ["num", "NonZeroU128"],
    ["num", "NonZeroUsize"],
];

const STANDARD_LIBRARY_ROOTS: [&str; 2] = ["core", "std"];

#[must_use]
pub(crate) fn is_std_copy_type(canonical: &CanonicalPath) -> bool {
    let [root, module, name] = canonical.segments() else {
        return false;
    };

    STANDARD_LIBRARY_ROOTS.contains(&root.as_str())
        && STANDARD_LIBRARY_COPY_TYPES
            .iter()
            .any(|known| known == &[module.as_str(), name.as_str()])
}

#[cfg(test)]
mod tests {
    use crate::canonical_path::CanonicalPath;

    use super::is_std_copy_type;

    fn is_copy(segments: &[&str]) -> bool {
        is_std_copy_type(&CanonicalPath::new(
            segments
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        ))
    }

    #[test]
    fn recognizes_every_known_standard_library_copy_type() {
        for [module, name] in super::STANDARD_LIBRARY_COPY_TYPES {
            assert!(is_copy(&["std", module, name]));
        }
    }

    #[test]
    fn recognizes_a_copy_type_reached_through_the_core_root() {
        assert!(is_copy(&["core", "num", "NonZeroU32"]));
    }

    #[test]
    fn rejects_a_standard_library_type_that_is_not_copy() {
        assert!(!is_copy(&["std", "string", "String"]));
    }

    #[test]
    fn rejects_a_type_under_a_root_that_is_not_the_standard_library() {
        assert!(!is_copy(&["nonzero", "num", "NonZeroU32"]));
    }

    #[test]
    fn rejects_a_path_that_is_not_three_segments_long() {
        assert!(!is_copy(&["u16"]));
    }
}
