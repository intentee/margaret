use std::collections::HashSet;

use syn::Path;

use crate::canonical_path::CanonicalPath;
use crate::module_imports::ModuleImports;

fn prelude_path(leaf: &str) -> Option<CanonicalPath> {
    match leaf {
        "String" => Some(CanonicalPath::new(vec![
            "std".to_string(),
            "string".to_string(),
            "String".to_string(),
        ])),
        "bool" => Some(CanonicalPath::new(vec!["bool".to_string()])),
        _ => None,
    }
}

#[must_use]
pub fn resolve_path(
    path: &Path,
    module_path: &[String],
    imports: &ModuleImports,
    item_paths: &HashSet<CanonicalPath>,
) -> Option<CanonicalPath> {
    let segments: Vec<String> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let leaf = segments.last()?;

    if segments.len() > 1 {
        return Some(CanonicalPath::new(segments));
    }

    if let Some(imported) = imports.resolve(leaf) {
        return Some(imported.clone());
    }

    let mut local_segments = module_path.to_vec();
    local_segments.push(leaf.clone());
    let local = CanonicalPath::new(local_segments);

    if item_paths.contains(&local) {
        return Some(local);
    }

    prelude_path(leaf)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use syn::parse_quote;

    use crate::canonical_path::CanonicalPath;
    use crate::module_imports::ModuleImports;

    use super::resolve_path;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn resolved(
        written: syn::Path,
        module: &[&str],
        imports: &ModuleImports,
        items: &HashSet<CanonicalPath>,
    ) -> Option<String> {
        let module_path: Vec<String> = module.iter().map(|segment| segment.to_string()).collect();

        resolve_path(&written, &module_path, imports, items).map(|resolved| resolved.to_string())
    }

    #[test]
    fn resolves_an_imported_leaf() {
        let mut imports = ModuleImports::default();
        imports.insert(
            "Greeter".to_string(),
            path(&["crate", "greeter", "Greeter"]),
        );

        assert_eq!(
            resolved(parse_quote!(Greeter), &["crate"], &imports, &HashSet::new()),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn resolves_a_multi_segment_path_as_written() {
        assert_eq!(
            resolved(
                parse_quote!(margaret_http::request::Request),
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new(),
            ),
            Some("margaret_http::request::Request".to_string())
        );
    }

    #[test]
    fn ignores_generic_arguments_on_the_leaf() {
        let mut imports = ModuleImports::default();
        imports.insert(
            "ValidationResult".to_string(),
            path(&[
                "margaret_validation",
                "validation_result",
                "ValidationResult",
            ]),
        );

        assert_eq!(
            resolved(
                parse_quote!(ValidationResult<Form>),
                &["crate"],
                &imports,
                &HashSet::new()
            ),
            Some("margaret_validation::validation_result::ValidationResult".to_string())
        );
    }

    #[test]
    fn resolves_a_same_module_defined_leaf() {
        let items = HashSet::from([path(&["crate", "models", "User"])]);

        assert_eq!(
            resolved(
                parse_quote!(User),
                &["crate", "models"],
                &ModuleImports::default(),
                &items
            ),
            Some("crate::models::User".to_string())
        );
    }

    #[test]
    fn a_local_definition_shadows_the_prelude() {
        let items = HashSet::from([path(&["crate", "String"])]);

        assert_eq!(
            resolved(
                parse_quote!(String),
                &["crate"],
                &ModuleImports::default(),
                &items
            ),
            Some("crate::String".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_string_to_the_prelude() {
        assert_eq!(
            resolved(
                parse_quote!(String),
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("std::string::String".to_string())
        );
    }

    #[test]
    fn resolves_a_bare_bool_to_the_prelude() {
        assert_eq!(
            resolved(
                parse_quote!(bool),
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("bool".to_string())
        );
    }

    #[test]
    fn returns_none_for_an_unresolvable_leaf() {
        assert_eq!(
            resolved(
                parse_quote!(Unknown),
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            None
        );
    }

    #[test]
    fn a_path_without_segments_is_unresolvable() {
        let empty = syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::new(),
        };

        assert_eq!(
            resolved(empty, &["crate"], &ModuleImports::default(), &HashSet::new()),
            None
        );
    }
}
