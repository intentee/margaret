use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;
use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::copy_path::copy_path;
use crate::is_copy_primitive::is_copy_primitive;
use crate::item_paths::ItemPaths;
use crate::module_imports::ModuleImports;
use crate::standard_library_item::StandardLibraryItem;

fn prelude_path(leaf: &str) -> Option<CanonicalPath> {
    match leaf {
        "Box" => Some(StandardLibraryItem::Box.std_path()),
        "Copy" => Some(copy_path()),
        "Option" => Some(StandardLibraryItem::Option.std_path()),
        "Send" => Some(StandardLibraryItem::Send.std_path()),
        "String" => Some(StandardLibraryItem::String.std_path()),
        "Sync" => Some(StandardLibraryItem::Sync.std_path()),
        "Vec" => Some(StandardLibraryItem::Vec.std_path()),
        primitive if is_copy_primitive(primitive) => {
            Some(CanonicalPath::new(vec![leaf.to_string()]))
        }
        _ => None,
    }
}

fn crate_root_of(module: &[String]) -> Vec<String> {
    module.iter().take(1).cloned().collect()
}

fn is_path_anchor(segment: &str) -> bool {
    matches!(segment, "crate" | "self" | "super")
}

fn joined(prefix: &[String], segment: &str) -> Vec<String> {
    let mut segments = prefix.to_vec();

    segments.push(segment.to_string());

    segments
}

fn anchored<'segments>(
    module: &[String],
    segments: &'segments [String],
) -> AnchoredSegments<'segments> {
    let mut base = module.to_vec();
    let mut rest = segments;

    while let Some((leading, trailing)) = rest.split_first() {
        match leading.as_str() {
            "self" => {}
            "super" => {
                base.pop();
            }
            _ => break,
        }

        rest = trailing;
    }

    AnchoredSegments { base, rest }
}

struct AnchoredSegments<'segments> {
    base: Vec<String>,
    rest: &'segments [String],
}

#[derive(Eq, Hash, PartialEq)]
struct FollowedImport {
    module: Vec<String>,
    name: String,
}

enum NameBinding {
    Bound(Vec<String>),
    Unbound,
}

#[derive(Default)]
pub struct PathResolver {
    definitions: ItemPaths,
    imports: HashMap<CanonicalPath, ModuleImports>,
}

impl PathResolver {
    pub(crate) fn new(
        definitions: ItemPaths,
        imports: HashMap<CanonicalPath, ModuleImports>,
    ) -> Self {
        Self {
            definitions,
            imports,
        }
    }

    #[must_use]
    pub fn resolve_path(&self, module: &[String], path: &Path) -> Option<CanonicalPath> {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();

        let mut followed = HashSet::new();

        match segments.as_slice() {
            [] => None,
            [name] if !is_path_anchor(name) => match self.bind(module, name, &mut followed) {
                NameBinding::Bound(bound) => Some(CanonicalPath::new(bound)),
                NameBinding::Unbound => prelude_path(name),
            },
            _ => Some(CanonicalPath::new(self.walk_from_anchor(
                module,
                &segments,
                &mut followed,
            ))),
        }
    }

    #[must_use]
    pub fn resolve_type(&self, module: &[String], declared: &Type) -> Option<CanonicalPath> {
        match declared {
            Type::Reference(reference) => self.resolve_type(module, &reference.elem),
            Type::Path(type_path) => self.resolve_path(module, &type_path.path),
            _ => None,
        }
    }

    pub(crate) fn extend(&mut self, other: Self) {
        let Self {
            definitions,
            imports,
        } = other;

        self.definitions.extend(definitions);
        self.imports.extend(imports);
    }

    fn bind(
        &self,
        module: &[String],
        name: &str,
        followed: &mut HashSet<FollowedImport>,
    ) -> NameBinding {
        let defined = joined(module, name);

        if self
            .definitions
            .contains(&CanonicalPath::new(defined.clone()))
        {
            return NameBinding::Bound(defined);
        }

        let Some(target) = self
            .imports
            .get(&CanonicalPath::new(module.to_vec()))
            .and_then(|imports| imports.resolve(name))
        else {
            return NameBinding::Unbound;
        };

        let import = FollowedImport {
            module: module.to_vec(),
            name: name.to_string(),
        };

        if !followed.insert(import) {
            return NameBinding::Unbound;
        }

        NameBinding::Bound(self.walk_from_anchor(module, target.segments(), followed))
    }

    fn walk(
        &self,
        root: Vec<String>,
        rest: &[String],
        followed: &mut HashSet<FollowedImport>,
    ) -> Vec<String> {
        let mut resolved = root;

        for segment in rest {
            resolved = match self.bind(&resolved, segment, followed) {
                NameBinding::Bound(bound) => bound,
                NameBinding::Unbound => joined(&resolved, segment),
            };
        }

        resolved
    }

    fn walk_from_anchor(
        &self,
        module: &[String],
        segments: &[String],
        followed: &mut HashSet<FollowedImport>,
    ) -> Vec<String> {
        match segments.split_first() {
            Some((leading, trailing)) if leading == "crate" => {
                self.walk(crate_root_of(module), trailing, followed)
            }
            Some((leading, trailing)) if !is_path_anchor(leading) => {
                match self.bind(module, leading, followed) {
                    NameBinding::Bound(root) => self.walk(root, trailing, followed),
                    NameBinding::Unbound => self.walk(vec![leading.clone()], trailing, followed),
                }
            }
            _ => {
                let AnchoredSegments { base, rest } = anchored(module, segments);

                self.walk(base, rest, followed)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use syn::Path;
    use syn::Type;
    use syn::parse_quote;
    use syn::punctuated::Punctuated;

    use super::PathResolver;
    use crate::canonical_path::CanonicalPath;
    use crate::module_imports::ModuleImports;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    fn module(segments: &[&str]) -> Vec<String> {
        segments.iter().map(ToString::to_string).collect()
    }

    struct Import<'fixture> {
        module: &'fixture [&'fixture str],
        name: &'fixture str,
        target: &'fixture [&'fixture str],
    }

    fn resolver(definitions: &[&[&str]], imports: &[Import]) -> PathResolver {
        let mut by_module: HashMap<CanonicalPath, ModuleImports> = HashMap::new();

        for Import {
            module,
            name,
            target,
        } in imports
        {
            by_module
                .entry(path(module))
                .or_default()
                .insert((*name).to_string(), path(target));
        }

        PathResolver::new(
            definitions
                .iter()
                .map(|definition| path(definition))
                .collect(),
            by_module,
        )
    }

    fn resolved(resolver: &PathResolver, written: &Path, in_module: &[&str]) -> Option<String> {
        resolver
            .resolve_path(&module(in_module), written)
            .map(|resolved| resolved.to_string())
    }

    fn resolved_type(resolver: &PathResolver, declared: &Type) -> Option<String> {
        resolver
            .resolve_type(&module(&["crate"]), declared)
            .map(|resolved| resolved.to_string())
    }

    #[test]
    fn resolves_an_imported_leaf() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "Greeter",
                target: &["crate", "greeter", "Greeter"],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(Greeter), &["crate"]),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn resolves_a_multi_segment_path_as_written() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(margaret_http::request::Request),
                &["crate"]
            ),
            Some("margaret_http::request::Request".to_string())
        );
    }

    #[test]
    fn ignores_generic_arguments_on_the_leaf() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "ValidationResult",
                target: &[
                    "margaret_validation",
                    "validation_result",
                    "ValidationResult",
                ],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(ValidationResult<Form>), &["crate"]),
            Some("margaret_validation::validation_result::ValidationResult".to_string())
        );
    }

    #[test]
    fn resolves_a_same_module_defined_leaf() {
        let resolver = resolver(&[&["crate", "models", "User"]], &[]);

        assert_eq!(
            resolved(&resolver, &parse_quote!(User), &["crate", "models"]),
            Some("crate::models::User".to_string())
        );
    }

    #[test]
    fn a_local_definition_shadows_the_prelude() {
        let resolver = resolver(&[&["crate", "String"]], &[]);

        assert_eq!(
            resolved(&resolver, &parse_quote!(String), &["crate"]),
            Some("crate::String".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_string_to_the_prelude() {
        assert_eq!(
            resolved(&PathResolver::default(), &parse_quote!(String), &["crate"]),
            Some("std::string::String".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_copy_primitive_to_the_prelude() {
        assert_eq!(
            resolved(&PathResolver::default(), &parse_quote!(u16), &["crate"]),
            Some("u16".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_copy_marker_to_the_prelude() {
        assert_eq!(
            resolved(&PathResolver::default(), &parse_quote!(Copy), &["crate"]),
            Some("std::marker::Copy".to_string())
        );
    }

    #[test]
    fn returns_none_for_an_unresolvable_leaf() {
        assert_eq!(
            resolved(&PathResolver::default(), &parse_quote!(Unknown), &["crate"]),
            None
        );
    }

    #[test]
    fn a_path_without_segments_is_unresolvable() {
        let empty = Path {
            leading_colon: None,
            segments: Punctuated::new(),
        };

        assert_eq!(resolved(&PathResolver::default(), &empty, &["crate"]), None);
    }

    #[test]
    fn resolves_a_crate_rooted_multi_segment_path_as_written() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(crate::greeter::Greeter),
                &["crate", "routes"]
            ),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn roots_the_crate_keyword_at_the_crate_of_the_resolving_module() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(crate::greeter::Greeter),
                &["storefront", "routes"]
            ),
            Some("storefront::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn an_imported_module_prefix_canonicalizes_to_the_direct_spelling() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate", "routes"],
                name: "greeter",
                target: &["crate", "greeter"],
            }],
        );

        let direct = resolved(
            &PathResolver::default(),
            &parse_quote!(crate::greeter::Greeter),
            &["crate", "routes"],
        );
        let aliased = resolved(
            &resolver,
            &parse_quote!(greeter::Greeter),
            &["crate", "routes"],
        );

        assert_eq!(aliased, direct);
        assert_eq!(aliased, Some("crate::greeter::Greeter".to_string()));
    }

    #[test]
    fn a_module_alias_canonicalizes_to_the_underlying_module() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "g",
                target: &["crate", "greeter"],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(g::Greeter), &["crate"]),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn resolves_a_self_prefixed_path_against_the_current_module() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(self::User),
                &["crate", "models"]
            ),
            Some("crate::models::User".to_string())
        );
    }

    #[test]
    fn resolves_a_super_prefixed_path_against_the_parent_module() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(super::Shared),
                &["crate", "models", "user"]
            ),
            Some("crate::models::Shared".to_string())
        );
    }

    #[test]
    fn resolves_consecutive_super_segments_against_the_grandparent_module() {
        assert_eq!(
            resolved(
                &PathResolver::default(),
                &parse_quote!(super::super::Shared),
                &["crate", "models", "user"]
            ),
            Some("crate::Shared".to_string())
        );
    }

    #[test]
    fn resolves_a_relative_submodule_path_against_the_current_module() {
        let resolver = resolver(
            &[
                &["crate", "models", "user"],
                &["crate", "models", "user", "User"],
            ],
            &[],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(user::User), &["crate", "models"]),
            Some("crate::models::user::User".to_string())
        );
    }

    #[test]
    fn follows_a_reexport_to_the_defining_item() {
        let resolver = resolver(
            &[
                &["crate", "models"],
                &["crate", "models", "user"],
                &["crate", "models", "user", "User"],
            ],
            &[Import {
                module: &["crate", "models"],
                name: "User",
                target: &["user", "User"],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(crate::models::User), &["crate"]),
            Some("crate::models::user::User".to_string())
        );
    }

    #[test]
    fn follows_chained_reexports_to_the_defining_item() {
        let resolver = resolver(
            &[
                &["crate", "models"],
                &["crate", "models", "user"],
                &["crate", "models", "user", "User"],
                &["crate", "prelude"],
            ],
            &[
                Import {
                    module: &["crate", "models"],
                    name: "User",
                    target: &["user", "User"],
                },
                Import {
                    module: &["crate", "prelude"],
                    name: "User",
                    target: &["crate", "models", "User"],
                },
                Import {
                    module: &["crate", "routes"],
                    name: "User",
                    target: &["crate", "prelude", "User"],
                },
            ],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(User), &["crate", "routes"]),
            Some("crate::models::user::User".to_string())
        );
    }

    #[test]
    fn follows_a_renamed_reexport_to_the_defining_item() {
        let resolver = resolver(
            &[
                &["crate", "models"],
                &["crate", "models", "User"],
                &["crate", "prelude"],
            ],
            &[Import {
                module: &["crate", "prelude"],
                name: "Member",
                target: &["crate", "models", "User"],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(crate::prelude::Member), &["crate"]),
            Some("crate::models::User".to_string())
        );
    }

    #[test]
    fn follows_a_module_reexport_before_the_remaining_segments() {
        let resolver = resolver(
            &[&["crate", "prelude"]],
            &[Import {
                module: &["crate", "prelude"],
                name: "request_input",
                target: &["margaret", "framework", "http_validation", "request_input"],
            }],
        );

        assert_eq!(
            resolved(
                &resolver,
                &parse_quote!(crate::prelude::request_input::RequestInput::Cookie),
                &["crate"]
            ),
            Some(
                "margaret::framework::http_validation::request_input::RequestInput::Cookie"
                    .to_string()
            )
        );
    }

    #[test]
    fn follows_the_parents_import_through_super() {
        let resolver = resolver(
            &[&["crate", "routes"], &["crate", "routes", "public"]],
            &[Import {
                module: &["crate", "routes"],
                name: "Store",
                target: &["crate", "stores", "Store"],
            }],
        );

        assert_eq!(
            resolved(
                &resolver,
                &parse_quote!(super::Store),
                &["crate", "routes", "public"]
            ),
            Some("crate::stores::Store".to_string())
        );
    }

    #[test]
    fn resolves_through_an_import_of_an_external_crate_name() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "serde",
                target: &["serde"],
            }],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(serde::Serialize), &["crate"]),
            Some("serde::Serialize".to_string())
        );
    }

    #[test]
    fn stops_following_an_import_cycle_at_the_repeated_import() {
        let resolver = resolver(
            &[&["crate", "first"], &["crate", "second"]],
            &[
                Import {
                    module: &["crate", "first"],
                    name: "Looped",
                    target: &["crate", "second", "Looped"],
                },
                Import {
                    module: &["crate", "second"],
                    name: "Looped",
                    target: &["crate", "first", "Looped"],
                },
            ],
        );

        assert_eq!(
            resolved(&resolver, &parse_quote!(crate::first::Looped), &["crate"]),
            Some("crate::first::Looped".to_string())
        );
    }

    #[test]
    fn peels_a_shared_reference() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "Routes",
                target: &["crate", "margaret", "routes", "Routes"],
            }],
        );

        assert_eq!(
            resolved_type(&resolver, &parse_quote!(&Routes)),
            Some("crate::margaret::routes::Routes".to_string())
        );
    }

    #[test]
    fn peels_a_mutable_reference() {
        let resolver = resolver(
            &[],
            &[Import {
                module: &["crate"],
                name: "Routes",
                target: &["crate", "routes", "Routes"],
            }],
        );

        assert_eq!(
            resolved_type(&resolver, &parse_quote!(&mut Routes)),
            Some("crate::routes::Routes".to_string())
        );
    }

    #[test]
    fn returns_none_for_a_non_path_type() {
        assert_eq!(
            resolved_type(&PathResolver::default(), &parse_quote!((u8, u8))),
            None
        );
    }
}
