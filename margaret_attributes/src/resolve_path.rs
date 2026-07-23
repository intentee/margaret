use std::collections::HashSet;

use syn::Path;

use crate::canonical_path::CanonicalPath;
use crate::is_copy_primitive::is_copy_primitive;
use crate::module_imports::ModuleImports;

fn prelude_path(leaf: &str) -> Option<CanonicalPath> {
    match leaf {
        "String" => Some(CanonicalPath::new(vec![
            "std".to_string(),
            "string".to_string(),
            "String".to_string(),
        ])),
        primitive if is_copy_primitive(primitive) => {
            Some(CanonicalPath::new(vec![leaf.to_string()]))
        }
        _ => None,
    }
}

fn rooted(prefix: &[String], rest: &[String]) -> CanonicalPath {
    let mut segments = prefix.to_vec();

    segments.extend(rest.iter().cloned());

    CanonicalPath::new(segments)
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
    let (leading, trailing) = segments.split_first()?;

    if leading == "crate" {
        return Some(CanonicalPath::new(segments));
    }

    if leading == "self" {
        return Some(rooted(module_path, trailing));
    }

    if leading == "super" {
        let parent = &module_path[..module_path.len().saturating_sub(1)];

        return Some(rooted(parent, trailing));
    }

    if let Some(imported) = imports.resolve(leading) {
        return Some(rooted(imported.segments(), trailing));
    }

    let local = rooted(module_path, &segments);

    if item_paths.contains(&local) {
        return Some(local);
    }

    if !trailing.is_empty() {
        return Some(CanonicalPath::new(segments));
    }

    prelude_path(leading)
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
    fn resolves_a_bare_copy_primitive_to_the_prelude() {
        assert_eq!(
            resolved(
                parse_quote!(u16),
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("u16".to_string())
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
            resolved(
                empty,
                &["crate"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            None
        );
    }

    #[test]
    fn resolves_a_crate_rooted_multi_segment_path_as_written() {
        assert_eq!(
            resolved(
                parse_quote!(crate::greeter::Greeter),
                &["crate", "routes"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn an_imported_module_prefix_canonicalizes_to_the_direct_spelling() {
        let mut imports = ModuleImports::default();
        imports.insert("greeter".to_string(), path(&["crate", "greeter"]));

        let direct = resolved(
            parse_quote!(crate::greeter::Greeter),
            &["crate"],
            &ModuleImports::default(),
            &HashSet::new(),
        );
        let aliased = resolved(
            parse_quote!(greeter::Greeter),
            &["crate"],
            &imports,
            &HashSet::new(),
        );

        assert_eq!(aliased, direct);
        assert_eq!(aliased, Some("crate::greeter::Greeter".to_string()));
    }

    #[test]
    fn a_module_alias_canonicalizes_to_the_underlying_module() {
        let mut imports = ModuleImports::default();
        imports.insert("g".to_string(), path(&["crate", "greeter"]));

        assert_eq!(
            resolved(
                parse_quote!(g::Greeter),
                &["crate"],
                &imports,
                &HashSet::new()
            ),
            Some("crate::greeter::Greeter".to_string())
        );
    }

    #[test]
    fn resolves_a_self_prefixed_path_against_the_current_module() {
        assert_eq!(
            resolved(
                parse_quote!(self::User),
                &["crate", "models"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("crate::models::User".to_string())
        );
    }

    #[test]
    fn resolves_a_super_prefixed_path_against_the_parent_module() {
        assert_eq!(
            resolved(
                parse_quote!(super::Shared),
                &["crate", "models", "user"],
                &ModuleImports::default(),
                &HashSet::new()
            ),
            Some("crate::models::Shared".to_string())
        );
    }

    #[test]
    fn resolves_a_relative_submodule_path_against_the_current_module() {
        let items = HashSet::from([path(&["crate", "models", "user", "User"])]);

        assert_eq!(
            resolved(
                parse_quote!(user::User),
                &["crate", "models"],
                &ModuleImports::default(),
                &items
            ),
            Some("crate::models::user::User".to_string())
        );
    }
}
