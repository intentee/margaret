use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::item_paths::ItemPaths;
use crate::module_imports::ModuleImports;
use crate::resolve_path::resolve_path;

#[must_use]
pub fn resolve_type(
    declared: &Type,
    module_path: &[String],
    imports: &ModuleImports,
    item_paths: &ItemPaths,
) -> Option<CanonicalPath> {
    match declared {
        Type::Reference(reference) => {
            resolve_type(&reference.elem, module_path, imports, item_paths)
        }
        Type::Path(type_path) => resolve_path(&type_path.path, module_path, imports, item_paths),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use syn::parse_quote;

    use crate::canonical_path::CanonicalPath;
    use crate::item_paths::ItemPaths;
    use crate::module_imports::ModuleImports;

    use super::resolve_type;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(
            segments
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }

    fn resolved(
        declared: &syn::Type,
        imports: &ModuleImports,
        items: &ItemPaths,
    ) -> Option<String> {
        resolve_type(declared, &["crate".to_string()], imports, items).map(|path| path.to_string())
    }

    #[test]
    fn peels_a_shared_reference() {
        let mut imports = ModuleImports::default();
        imports.insert(
            "Routes".to_string(),
            path(&["crate", "margaret", "routes", "Routes"]),
        );

        assert_eq!(
            resolved(&parse_quote!(&Routes), &imports, &ItemPaths::default()),
            Some("crate::margaret::routes::Routes".to_string())
        );
    }

    #[test]
    fn peels_a_mutable_reference() {
        let mut imports = ModuleImports::default();
        imports.insert("Routes".to_string(), path(&["crate", "routes", "Routes"]));

        assert_eq!(
            resolved(&parse_quote!(&mut Routes), &imports, &ItemPaths::default()),
            Some("crate::routes::Routes".to_string())
        );
    }

    #[test]
    fn returns_none_for_a_non_path_type() {
        assert_eq!(
            resolved(
                &parse_quote!((u8, u8)),
                &ModuleImports::default(),
                &ItemPaths::default()
            ),
            None
        );
    }
}
