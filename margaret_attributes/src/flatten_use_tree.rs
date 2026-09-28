use syn::UseTree;

use crate::canonical_path::CanonicalPath;
use crate::flattened_import::FlattenedImport;

struct UseTreeAnchor<'tree> {
    prefix: Vec<String>,
    tree: &'tree UseTree,
}

fn resolve_anchor<'tree>(tree: &'tree UseTree, module_path: &[String]) -> UseTreeAnchor<'tree> {
    let mut base = module_path.to_vec();
    let mut current = tree;
    let mut relative = false;

    loop {
        match current {
            UseTree::Path(use_path) if use_path.ident == "self" => {
                relative = true;
                current = &use_path.tree;
            }
            UseTree::Path(use_path) if use_path.ident == "super" => {
                relative = true;
                base.truncate(base.len().saturating_sub(1));
                current = &use_path.tree;
            }
            _ => {
                return UseTreeAnchor {
                    prefix: if relative { base } else { Vec::new() },
                    tree: current,
                };
            }
        }
    }
}

fn flatten(tree: &UseTree, prefix: Vec<String>) -> Vec<FlattenedImport> {
    match tree {
        UseTree::Path(use_path) => {
            let mut child_prefix = prefix;
            child_prefix.push(use_path.ident.to_string());

            flatten(&use_path.tree, child_prefix)
        }
        UseTree::Name(use_name) => {
            let name = use_name.ident.to_string();

            if name == "self" {
                return Vec::new();
            }

            let mut segments = prefix;
            segments.push(name.clone());

            vec![FlattenedImport {
                name,
                path: CanonicalPath::new(segments),
            }]
        }
        UseTree::Rename(use_rename) => {
            let mut segments = prefix;
            segments.push(use_rename.ident.to_string());

            vec![FlattenedImport {
                name: use_rename.rename.to_string(),
                path: CanonicalPath::new(segments),
            }]
        }
        UseTree::Group(use_group) => use_group
            .items
            .iter()
            .flat_map(|item| flatten(item, prefix.clone()))
            .collect(),
        UseTree::Glob(_) => Vec::new(),
    }
}

pub(crate) fn flatten_use_tree(tree: &UseTree, module_path: &[String]) -> Vec<FlattenedImport> {
    let UseTreeAnchor { prefix, tree } = resolve_anchor(tree, module_path);

    flatten(tree, prefix)
}

#[cfg(test)]
mod tests {
    use syn::ItemUse;
    use syn::parse_quote;

    use super::flatten_use_tree;

    #[derive(Debug, PartialEq, Eq)]
    struct Entry {
        name: String,
        path: String,
    }

    fn entry(name: &str, path: &str) -> Entry {
        Entry {
            name: name.to_string(),
            path: path.to_string(),
        }
    }

    fn entries(tree: &ItemUse, module_path: &[&str]) -> Vec<Entry> {
        let module: Vec<String> = module_path.iter().map(ToString::to_string).collect();

        flatten_use_tree(&tree.tree, &module)
            .into_iter()
            .map(|import| Entry {
                name: import.name,
                path: import.path.to_string(),
            })
            .collect()
    }

    #[test]
    fn resolves_an_external_crate_import_as_written() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use margaret_http::request::Request;
                ),
                &["crate"]
            ),
            vec![entry("Request", "margaret_http::request::Request")]
        );
    }

    #[test]
    fn resolves_a_crate_rooted_import_as_written() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use crate::margaret::routes::Routes;
                ),
                &["crate", "routes"]
            ),
            vec![entry("Routes", "crate::margaret::routes::Routes")]
        );
    }

    #[test]
    fn resolves_a_self_import_against_the_current_module() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use self::User;
                ),
                &["crate", "models"]
            ),
            vec![entry("User", "crate::models::User")]
        );
    }

    #[test]
    fn resolves_a_super_import_against_the_parent_module() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use super::Shared;
                ),
                &["crate", "models", "user"]
            ),
            vec![entry("Shared", "crate::models::Shared")]
        );
    }

    #[test]
    fn resolves_nested_super_imports_against_the_grandparent_module() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use super::super::Root;
                ),
                &["crate", "models", "user"]
            ),
            vec![entry("Root", "crate::Root")]
        );
    }

    #[test]
    fn flattens_a_rename_under_its_alias() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use foo::Bar as Renamed;
                ),
                &["crate"]
            ),
            vec![entry("Renamed", "foo::Bar")]
        );
    }

    #[test]
    fn flattens_a_group() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use foo::{Bar, baz::Qux};
                ),
                &["crate"]
            ),
            vec![entry("Bar", "foo::Bar"), entry("Qux", "foo::baz::Qux"),]
        );
    }

    #[test]
    fn skips_the_self_group_member() {
        assert_eq!(
            entries(
                &parse_quote!(
                    use foo::{self, Bar};
                ),
                &["crate"]
            ),
            vec![entry("Bar", "foo::Bar")]
        );
    }

    #[test]
    fn ignores_a_glob() {
        assert!(
            entries(
                &parse_quote!(
                    use foo::*;
                ),
                &["crate"]
            )
            .is_empty()
        );
    }
}
