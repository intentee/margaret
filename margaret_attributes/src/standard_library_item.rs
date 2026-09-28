use crate::canonical_path::CanonicalPath;

const CORE_ROOTS: [&str; 2] = ["core", "std"];

const ALLOC_ROOTS: [&str; 2] = ["alloc", "std"];

struct StandardLibraryLocation {
    module: &'static str,
    name: &'static str,
    roots: &'static [&'static str; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StandardLibraryItem {
    Arc,
    Box,
    Option,
    Rc,
    Send,
    String,
    Sync,
    Vec,
}

impl StandardLibraryItem {
    #[must_use]
    pub fn from_canonical(canonical: &CanonicalPath) -> Option<Self> {
        let [root, module, name] = canonical.segments() else {
            return None;
        };

        [
            Self::Arc,
            Self::Box,
            Self::Option,
            Self::Rc,
            Self::Send,
            Self::String,
            Self::Sync,
            Self::Vec,
        ]
        .into_iter()
        .find(|item| {
            let StandardLibraryLocation {
                module: item_module,
                name: item_name,
                roots,
            } = item.location();

            roots.contains(&root.as_str()) && item_module == module && item_name == name
        })
    }

    #[must_use]
    pub fn std_path(self) -> CanonicalPath {
        let StandardLibraryLocation { module, name, .. } = self.location();

        CanonicalPath::new(vec![
            "std".to_string(),
            module.to_string(),
            name.to_string(),
        ])
    }

    fn location(self) -> StandardLibraryLocation {
        match self {
            Self::Arc => StandardLibraryLocation {
                module: "sync",
                name: "Arc",
                roots: &ALLOC_ROOTS,
            },
            Self::Box => StandardLibraryLocation {
                module: "boxed",
                name: "Box",
                roots: &ALLOC_ROOTS,
            },
            Self::Option => StandardLibraryLocation {
                module: "option",
                name: "Option",
                roots: &CORE_ROOTS,
            },
            Self::Rc => StandardLibraryLocation {
                module: "rc",
                name: "Rc",
                roots: &ALLOC_ROOTS,
            },
            Self::Send => StandardLibraryLocation {
                module: "marker",
                name: "Send",
                roots: &CORE_ROOTS,
            },
            Self::String => StandardLibraryLocation {
                module: "string",
                name: "String",
                roots: &ALLOC_ROOTS,
            },
            Self::Sync => StandardLibraryLocation {
                module: "marker",
                name: "Sync",
                roots: &CORE_ROOTS,
            },
            Self::Vec => StandardLibraryLocation {
                module: "vec",
                name: "Vec",
                roots: &ALLOC_ROOTS,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StandardLibraryItem;
    use crate::canonical_path::CanonicalPath;

    fn recognized(segments: &[&str]) -> Option<StandardLibraryItem> {
        StandardLibraryItem::from_canonical(&CanonicalPath::new(
            segments.iter().map(ToString::to_string).collect(),
        ))
    }

    #[test]
    fn recognizes_every_item_by_its_std_path() {
        for item in [
            StandardLibraryItem::Arc,
            StandardLibraryItem::Box,
            StandardLibraryItem::Option,
            StandardLibraryItem::Rc,
            StandardLibraryItem::Send,
            StandardLibraryItem::String,
            StandardLibraryItem::Sync,
            StandardLibraryItem::Vec,
        ] {
            assert_eq!(
                StandardLibraryItem::from_canonical(&item.std_path()),
                Some(item)
            );
        }
    }

    #[test]
    fn recognizes_an_item_through_the_crate_that_defines_it() {
        assert_eq!(
            recognized(&["core", "option", "Option"]),
            Some(StandardLibraryItem::Option)
        );
        assert_eq!(
            recognized(&["alloc", "sync", "Arc"]),
            Some(StandardLibraryItem::Arc)
        );
    }

    #[test]
    fn rejects_an_item_through_a_crate_that_does_not_define_it() {
        assert_eq!(recognized(&["alloc", "option", "Option"]), None);
        assert_eq!(recognized(&["core", "sync", "Arc"]), None);
    }

    #[test]
    fn rejects_a_same_named_item_of_another_crate() {
        assert_eq!(recognized(&["pointers", "sync", "Arc"]), None);
        assert_eq!(recognized(&["crate", "Option"]), None);
    }
}
