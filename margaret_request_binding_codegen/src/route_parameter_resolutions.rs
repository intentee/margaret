use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_container::is_singleton::is_singleton;

use crate::request_binding_error::RequestBindingError;
use crate::route_parameter_binder::RouteParameterBinder;
use crate::route_parameter_resolution::RouteParameterResolution;

fn associated_model(index: &AttributeIndex, item: &IndexedItem) -> Option<CanonicalPath> {
    item.trait_impls().iter().find_map(|trait_impl| {
        let associated_type = trait_impl.associated_type("Model")?;
        let resolved = index.resolve_module_type(trait_impl.module_path(), associated_type.ty())?;

        index
            .struct_identifier(&resolved)
            .is_some()
            .then_some(resolved)
    })
}

fn string_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "std".to_string(),
        "string".to_string(),
        "String".to_string(),
    ])
}

fn insert_binder(
    index: &AttributeIndex,
    item: &IndexedItem,
    resolutions: &mut HashMap<CanonicalPath, RouteParameterResolution>,
) -> Result<(), RequestBindingError> {
    let provider = item.canonical_path().clone();
    let Some(identifier) = index.struct_identifier(&provider) else {
        return Err(RequestBindingError::RouteParameterBinderNotAStruct {
            binder: provider.to_string(),
        });
    };

    if !is_singleton(item) {
        return Err(RequestBindingError::RouteParameterBinderRequiresSingleton {
            binder: provider.to_string(),
        });
    }

    let field = identifier.field().to_string();
    let model = associated_model(index, item).ok_or_else(|| {
        RequestBindingError::RouteParameterBinderModel {
            binder: provider.to_string(),
        }
    })?;

    match resolutions.get(&model) {
        Some(RouteParameterResolution::Binder(existing)) => {
            return Err(RequestBindingError::AmbiguousRouteParameterBinder {
                model: model.to_string(),
                first: existing.provider.to_string(),
                second: provider.to_string(),
            });
        }
        Some(RouteParameterResolution::Value) => {
            return Err(RequestBindingError::ConflictingRouteParameterResolution {
                value_type: model.to_string(),
                binder: provider.to_string(),
            });
        }
        None => {}
    }

    resolutions.insert(
        model,
        RouteParameterResolution::Binder(RouteParameterBinder { field, provider }),
    );

    Ok(())
}

fn insert_value(
    item: &IndexedItem,
    resolutions: &mut HashMap<CanonicalPath, RouteParameterResolution>,
) -> Result<(), RequestBindingError> {
    let value_type = item.canonical_path().clone();

    if !item.kind().is_struct() && !item.kind().is_enum() {
        return Err(RequestBindingError::RouteParameterValueNotAStructOrEnum {
            value_type: value_type.to_string(),
        });
    }

    if let Some(RouteParameterResolution::Binder(existing)) = resolutions.get(&value_type) {
        return Err(RequestBindingError::ConflictingRouteParameterResolution {
            value_type: value_type.to_string(),
            binder: existing.provider.to_string(),
        });
    }

    resolutions.insert(value_type, RouteParameterResolution::Value);

    Ok(())
}

/// # Errors
///
/// Returns `RequestBindingError` propagated from the work it performs.
pub fn route_parameter_resolutions(
    index: &AttributeIndex,
) -> Result<HashMap<CanonicalPath, RouteParameterResolution>, RequestBindingError> {
    let mut resolutions = HashMap::from([(string_path(), RouteParameterResolution::Value)]);

    for item in index.items() {
        if item.has_framework_attribute(FrameworkAttribute::ProvidesRouteParameter) {
            insert_binder(index, item, &mut resolutions)?;
        }

        if item.has_framework_attribute(FrameworkAttribute::RouteParameterValue) {
            insert_value(item, &mut resolutions)?;
        }
    }

    Ok(resolutions)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;

    use super::route_parameter_resolutions;
    use crate::request_binding_error::RequestBindingError;
    use crate::route_parameter_binder::RouteParameterBinder;
    use crate::route_parameter_resolution::RouteParameterResolution;

    const BINDER: &str = "struct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n";

    #[derive(Debug, Eq, PartialEq)]
    enum ResolvedBy {
        Binder(String),
        Nothing,
        Value,
    }

    fn resolutions_for(
        lib_source: &str,
    ) -> Result<HashMap<CanonicalPath, RouteParameterResolution>, RequestBindingError> {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        route_parameter_resolutions(
            &AttributeIndexBuilder::new()
                .index_crate(&CrateRoot::new("crate", source_directory))
                .expect("the crate is indexed")
                .build(),
        )
    }

    fn rejection_for(lib_source: &str) -> String {
        resolutions_for(lib_source)
            .err()
            .expect("the crate is rejected")
            .to_string()
    }

    fn resolved_by(lib_source: &str, segments: &[&str]) -> ResolvedBy {
        let mut resolutions = resolutions_for(lib_source).expect("the crate is accepted");

        match resolutions.remove(&CanonicalPath::new(
            segments.iter().map(std::string::ToString::to_string).collect(),
        )) {
            Some(RouteParameterResolution::Binder(RouteParameterBinder { provider, .. })) => {
                ResolvedBy::Binder(provider.to_string())
            }
            Some(RouteParameterResolution::Value) => ResolvedBy::Value,
            None => ResolvedBy::Nothing,
        }
    }

    #[test]
    fn resolves_a_string_route_parameter_as_a_value_without_any_declaration() {
        assert_eq!(
            resolved_by("struct Nothing;\n", &["std", "string", "String"]),
            ResolvedBy::Value
        );
    }

    #[test]
    fn resolves_nothing_for_a_type_that_declares_no_route_parameter_role() {
        assert_eq!(
            resolved_by("struct Nothing;\n", &["crate", "Nothing"]),
            ResolvedBy::Nothing
        );
    }

    #[test]
    fn resolves_a_value_struct_as_a_value() {
        assert_eq!(
            resolved_by(
                "#[route_parameter_value]\nstruct ArticleSlug(String);\n",
                &["crate", "ArticleSlug"]
            ),
            ResolvedBy::Value
        );
    }

    #[test]
    fn resolves_a_value_enum_as_a_value() {
        assert_eq!(
            resolved_by(
                "#[route_parameter_value]\nenum ArticleStatus {\n    Draft,\n    Published,\n}\n",
                &["crate", "ArticleStatus"]
            ),
            ResolvedBy::Value
        );
    }

    #[test]
    fn resolves_a_binder_model_through_its_provider() {
        assert_eq!(
            resolved_by(BINDER, &["crate", "Article"]),
            ResolvedBy::Binder("crate::ArticleStore".to_string())
        );
    }

    #[test]
    fn rejects_a_value_declaration_on_something_that_is_neither_a_struct_nor_an_enum() {
        assert!(
            rejection_for("#[route_parameter_value]\ntrait ArticleSlug {}\n")
                .contains("is only supported on structs and enums")
        );
    }

    #[test]
    fn rejects_a_binder_whose_model_is_already_declared_as_a_value() {
        assert!(
            rejection_for(
                "#[route_parameter_value]\nstruct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("is both a #[route_parameter_value] and the model")
        );
    }

    #[test]
    fn rejects_a_value_declaration_for_a_type_that_a_binder_already_provides() {
        assert!(
            rejection_for(
                "#[route_parameter_value]\nstruct Zebra;\n\n#[singleton]\n#[provides_route_parameter]\nstruct AardvarkStore;\n\nimpl HttpRouteParameterBinder for AardvarkStore {\n    type Model = Zebra;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Zebra>> {}\n}\n"
            )
            .contains("is both a #[route_parameter_value] and the model")
        );
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        assert!(
            rejection_for(
                "struct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct First;\n\nimpl HttpRouteParameterBinder for First {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n\n#[singleton]\n#[provides_route_parameter]\nstruct Second;\n\nimpl HttpRouteParameterBinder for Second {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("has more than one route parameter binder")
        );
    }

    #[test]
    fn rejects_a_binder_that_is_not_a_struct() {
        assert!(
            rejection_for(
                "struct Article;\n\n#[provides_route_parameter]\nenum ArticleStore {}\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("is only supported on structs")
        );
    }

    #[test]
    fn rejects_a_binder_that_is_not_a_singleton() {
        assert!(
            rejection_for(
                "struct Article;\n\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("must also be declared as a #[singleton]")
        );
    }

    #[test]
    fn rejects_a_binder_without_a_model_that_resolves_to_a_struct() {
        assert!(
            rejection_for("#[singleton]\n#[provides_route_parameter]\nstruct Bare;\n")
                .contains("has no `type Model = <struct>` associated type")
        );
    }
}
