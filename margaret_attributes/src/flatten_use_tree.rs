use syn::UseTree;

use crate::canonical_path::CanonicalPath;

fn resolve_anchor<'tree>(
    tree: &'tree UseTree,
    module_path: &[String],
) -> (&'tree UseTree, Vec<String>) {
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
            _ => return (current, if relative { base } else { Vec::new() }),
        }
    }
}

fn flatten(tree: &UseTree, prefix: Vec<String>) -> Vec<(String, CanonicalPath)> {
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

            vec![(name, CanonicalPath::new(segments))]
        }
        UseTree::Rename(use_rename) => {
            let mut segments = prefix;
            segments.push(use_rename.ident.to_string());

            vec![(use_rename.rename.to_string(), CanonicalPath::new(segments))]
        }
        UseTree::Group(use_group) => use_group
            .items
            .iter()
            .flat_map(|item| flatten(item, prefix.clone()))
            .collect(),
        UseTree::Glob(_) => Vec::new(),
    }
}

pub(crate) fn flatten_use_tree(
    tree: &UseTree,
    module_path: &[String],
) -> Vec<(String, CanonicalPath)> {
    let (stripped, prefix) = resolve_anchor(tree, module_path);

    flatten(stripped, prefix)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::flatten_use_tree;

    fn entries(tree: syn::ItemUse, module_path: &[&str]) -> Vec<(String, String)> {
        let module: Vec<String> = module_path
            .iter()
            .map(|segment| segment.to_string())
            .collect();

        flatten_use_tree(&tree.tree, &module)
            .into_iter()
            .map(|(name, path)| (name, path.to_string()))
            .collect()
    }

    #[test]
    fn resolves_an_external_crate_import_as_written() {
        assert_eq!(
            entries(
                parse_quote!(
                    use margaret_http::request::Request;
                ),
                &["crate"]
            ),
            vec![(
                "Request".to_string(),
                "margaret_http::request::Request".to_string()
            )]
        );
    }

    #[test]
    fn resolves_a_crate_rooted_import_as_written() {
        assert_eq!(
            entries(
                parse_quote!(
                    use crate::margaret::routes::Routes;
                ),
                &["crate", "routes"]
            ),
            vec![(
                "Routes".to_string(),
                "crate::margaret::routes::Routes".to_string()
            )]
        );
    }

    #[test]
    fn resolves_a_self_import_against_the_current_module() {
        assert_eq!(
            entries(
                parse_quote!(
                    use self::User;
                ),
                &["crate", "models"]
            ),
            vec![("User".to_string(), "crate::models::User".to_string())]
        );
    }

    #[test]
    fn resolves_a_super_import_against_the_parent_module() {
        assert_eq!(
            entries(
                parse_quote!(
                    use super::Shared;
                ),
                &["crate", "models", "user"]
            ),
            vec![("Shared".to_string(), "crate::models::Shared".to_string())]
        );
    }

    #[test]
    fn resolves_nested_super_imports_against_the_grandparent_module() {
        assert_eq!(
            entries(
                parse_quote!(
                    use super::super::Root;
                ),
                &["crate", "models", "user"]
            ),
            vec![("Root".to_string(), "crate::Root".to_string())]
        );
    }

    #[test]
    fn flattens_a_rename_under_its_alias() {
        assert_eq!(
            entries(
                parse_quote!(
                    use foo::Bar as Renamed;
                ),
                &["crate"]
            ),
            vec![("Renamed".to_string(), "foo::Bar".to_string())]
        );
    }

    #[test]
    fn flattens_a_group() {
        assert_eq!(
            entries(
                parse_quote!(
                    use foo::{Bar, baz::Qux};
                ),
                &["crate"]
            ),
            vec![
                ("Bar".to_string(), "foo::Bar".to_string()),
                ("Qux".to_string(), "foo::baz::Qux".to_string()),
            ]
        );
    }

    #[test]
    fn skips_the_self_group_member() {
        assert_eq!(
            entries(
                parse_quote!(
                    use foo::{self, Bar};
                ),
                &["crate"]
            ),
            vec![("Bar".to_string(), "foo::Bar".to_string())]
        );
    }

    #[test]
    fn ignores_a_glob() {
        assert!(
            entries(
                parse_quote!(
                    use foo::*;
                ),
                &["crate"]
            )
            .is_empty()
        );
    }
}
