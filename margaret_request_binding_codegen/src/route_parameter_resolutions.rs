use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_trait_impl::IndexedTraitImpl;
use margaret_attributes::standard_library_item::StandardLibraryItem;
use margaret_container::is_singleton::is_singleton;
use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;

use crate::request_binding_error::RequestBindingError;
use crate::route_database::RouteDatabase;
use crate::route_model::RouteModel;
use crate::route_model_key::RouteModelKey;
use crate::route_model_resolution::RouteModelResolution;
use crate::route_parameter_binder::RouteParameterBinder;
use crate::route_parameter_conversion::RouteParameterConversion;
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

fn converts_from_string(index: &AttributeIndex, trait_impl: &IndexedTraitImpl) -> bool {
    let module = trait_impl.module_path();
    let trait_path = trait_impl.trait_path();
    let converted_from = trait_path
        .segments
        .last()
        .and_then(single_generic_argument)
        .and_then(|argument| index.resolve_module_type(module, argument));

    index
        .resolve_module_path(module, trait_path)
        .and_then(|resolved| StandardLibraryItem::from_canonical(&resolved))
        == Some(StandardLibraryItem::From)
        && converted_from == Some(StandardLibraryItem::String.std_path())
}

fn value_conversion(index: &AttributeIndex, item: &IndexedItem) -> RouteParameterConversion {
    if item
        .trait_impls()
        .iter()
        .any(|trait_impl| converts_from_string(index, trait_impl))
    {
        RouteParameterConversion::Infallible
    } else {
        RouteParameterConversion::Fallible
    }
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
        Some(RouteParameterResolution::Model(_)) => {
            return Err(RequestBindingError::RouteModelWithBinder {
                model: model.to_string(),
                binder: provider.to_string(),
            });
        }
        Some(RouteParameterResolution::Value(_)) => {
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
    index: &AttributeIndex,
    item: &IndexedItem,
    resolutions: &mut HashMap<CanonicalPath, RouteParameterResolution>,
) -> Result<(), RequestBindingError> {
    let value_type = item.canonical_path().clone();

    if !item.kind().is_struct() && !item.kind().is_enum() {
        return Err(RequestBindingError::RouteParameterValueNotAStructOrEnum {
            value_type: value_type.to_string(),
        });
    }

    match resolutions.get(&value_type) {
        Some(RouteParameterResolution::Binder(existing)) => {
            return Err(RequestBindingError::ConflictingRouteParameterResolution {
                value_type: value_type.to_string(),
                binder: existing.provider.to_string(),
            });
        }
        Some(RouteParameterResolution::Model(_)) => {
            return Err(RequestBindingError::RouteModelDeclaredAsValue {
                model: value_type.to_string(),
            });
        }
        Some(RouteParameterResolution::Value(_)) | None => {}
    }

    resolutions.insert(
        value_type,
        RouteParameterResolution::Value(value_conversion(index, item)),
    );

    Ok(())
}

fn model_resolution(primary_key: RouteModelKey, database: &RouteDatabase) -> RouteModelResolution {
    match primary_key {
        RouteModelKey::Composite => RouteModelResolution::CompositePrimaryKey,
        RouteModelKey::Single => match database {
            RouteDatabase::Declared(binder) => {
                RouteModelResolution::Bindable(RouteParameterBinder {
                    field: binder.field.clone(),
                    provider: binder.provider.clone(),
                })
            }
            RouteDatabase::Undeclared => RouteModelResolution::DatabaseUndeclared,
        },
    }
}

/// # Errors
///
/// Returns `RequestBindingError` propagated from the work it performs.
pub fn route_parameter_resolutions(
    index: &AttributeIndex,
    route_models: &[RouteModel],
    database: &RouteDatabase,
) -> Result<HashMap<CanonicalPath, RouteParameterResolution>, RequestBindingError> {
    let mut resolutions = HashMap::from([(
        StandardLibraryItem::String.std_path(),
        RouteParameterResolution::Value(RouteParameterConversion::Identity),
    )]);

    for RouteModel {
        loaded,
        primary_key,
    } in route_models
    {
        resolutions.insert(
            loaded.clone(),
            RouteParameterResolution::Model(model_resolution(*primary_key, database)),
        );
    }

    for item in index.items() {
        if item.has_framework_attribute(FrameworkAttribute::ProvidesRouteParameter) {
            insert_binder(index, item, &mut resolutions)?;
        }

        if item.has_framework_attribute(FrameworkAttribute::RouteParameterValue) {
            insert_value(index, item, &mut resolutions)?;
        }
    }

    Ok(resolutions)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::route_parameter_resolutions;
    use crate::request_binding_error::RequestBindingError;
    use crate::route_database::RouteDatabase;
    use crate::route_model::RouteModel;
    use crate::route_model_key::RouteModelKey;
    use crate::route_model_resolution::RouteModelResolution;
    use crate::route_parameter_binder::RouteParameterBinder;
    use crate::route_parameter_conversion::RouteParameterConversion;
    use crate::route_parameter_resolution::RouteParameterResolution;

    const BINDER: &str = "use margaret::framework::macros::provides_route_parameter;\nuse margaret::framework::macros::singleton;\nstruct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n";

    #[derive(Debug, Eq, PartialEq)]
    enum ResolvedBy {
        Binder(String),
        CompositePrimaryKey,
        Database(String),
        DatabaseUndeclared,
        Nothing,
        Value(RouteParameterConversion),
    }

    fn article() -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), "Article".to_string()])
    }

    fn database() -> RouteDatabase {
        RouteDatabase::Declared(RouteParameterBinder {
            field: "framework_database".to_string(),
            provider: CanonicalPath::new(vec!["margaret".to_string(), "Database".to_string()]),
        })
    }

    fn model_resolutions_for(
        lib_source: &str,
        primary_key: RouteModelKey,
        database: &RouteDatabase,
    ) -> Result<HashMap<CanonicalPath, RouteParameterResolution>, RequestBindingError> {
        route_parameter_resolutions(
            &IndexedSource::new(lib_source).index,
            &[RouteModel {
                loaded: article(),
                primary_key,
            }],
            database,
        )
    }

    fn resolutions_for(
        lib_source: &str,
    ) -> Result<HashMap<CanonicalPath, RouteParameterResolution>, RequestBindingError> {
        route_parameter_resolutions(
            &IndexedSource::new(lib_source).index,
            &[],
            &RouteDatabase::Undeclared,
        )
    }

    fn resolved(resolution: Option<RouteParameterResolution>) -> ResolvedBy {
        match resolution {
            Some(RouteParameterResolution::Binder(RouteParameterBinder { provider, .. })) => {
                ResolvedBy::Binder(provider.to_string())
            }
            Some(RouteParameterResolution::Model(RouteModelResolution::Bindable(
                RouteParameterBinder { field, .. },
            ))) => ResolvedBy::Database(field),
            Some(RouteParameterResolution::Model(RouteModelResolution::CompositePrimaryKey)) => {
                ResolvedBy::CompositePrimaryKey
            }
            Some(RouteParameterResolution::Model(RouteModelResolution::DatabaseUndeclared)) => {
                ResolvedBy::DatabaseUndeclared
            }
            Some(RouteParameterResolution::Value(conversion)) => ResolvedBy::Value(conversion),
            None => ResolvedBy::Nothing,
        }
    }

    fn model_resolved_by(primary_key: RouteModelKey, database: &RouteDatabase) -> ResolvedBy {
        resolved(
            model_resolutions_for("struct Article;\n", primary_key, database)
                .expect("the crate is accepted")
                .remove(&article()),
        )
    }

    fn rejection_for(lib_source: &str) -> String {
        resolutions_for(lib_source)
            .err()
            .expect("the crate is rejected")
            .to_string()
    }

    fn resolved_by(lib_source: &str, segments: &[&str]) -> ResolvedBy {
        resolved(
            resolutions_for(lib_source)
                .expect("the crate is accepted")
                .remove(&CanonicalPath::new(
                    segments.iter().map(ToString::to_string).collect(),
                )),
        )
    }

    #[test]
    fn resolves_a_string_route_parameter_as_a_value_without_any_declaration() {
        assert_eq!(
            resolved_by("struct Nothing;\n", &["std", "string", "String"]),
            ResolvedBy::Value(RouteParameterConversion::Identity)
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
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct ArticleSlug(String);\n",
                &["crate", "ArticleSlug"]
            ),
            ResolvedBy::Value(RouteParameterConversion::Fallible)
        );
    }

    #[test]
    fn resolves_a_value_enum_as_a_value() {
        assert_eq!(
            resolved_by(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nenum ArticleStatus {\n    Draft,\n    Published,\n}\n",
                &["crate", "ArticleStatus"]
            ),
            ResolvedBy::Value(RouteParameterConversion::Fallible)
        );
    }

    #[test]
    fn converts_a_value_from_a_string_infallibly_when_it_implements_from_string() {
        assert_eq!(
            resolved_by(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct ArticleSlug(String);\n\nimpl From<String> for ArticleSlug {\n    fn from(value: String) -> Self {}\n}\n",
                &["crate", "ArticleSlug"],
            ),
            ResolvedBy::Value(RouteParameterConversion::Infallible)
        );
    }

    #[test]
    fn converts_a_value_from_a_string_infallibly_through_a_qualified_from_string() {
        assert_eq!(
            resolved_by(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct ArticleSlug(String);\n\nimpl std::convert::From<std::string::String> for ArticleSlug {\n    fn from(value: String) -> Self {}\n}\n",
                &["crate", "ArticleSlug"],
            ),
            ResolvedBy::Value(RouteParameterConversion::Infallible)
        );
    }

    #[test]
    fn converts_a_value_fallibly_when_it_only_implements_try_from_string() {
        assert_eq!(
            resolved_by(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct PublishedYear(u16);\n\nimpl TryFrom<String> for PublishedYear {\n    type Error = std::num::ParseIntError;\n    fn try_from(value: String) -> Result<Self, Self::Error> {}\n}\n",
                &["crate", "PublishedYear"],
            ),
            ResolvedBy::Value(RouteParameterConversion::Fallible)
        );
    }

    #[test]
    fn converts_a_value_fallibly_when_its_from_takes_another_type_than_a_string() {
        assert_eq!(
            resolved_by(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct PublishedYear(u16);\n\nimpl From<u16> for PublishedYear {\n    fn from(value: u16) -> Self {}\n}\n\nimpl TryFrom<String> for PublishedYear {\n    type Error = std::num::ParseIntError;\n    fn try_from(value: String) -> Result<Self, Self::Error> {}\n}\n",
                &["crate", "PublishedYear"],
            ),
            ResolvedBy::Value(RouteParameterConversion::Fallible)
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
            rejection_for("use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\ntrait ArticleSlug {}\n")
                .contains("is only supported on structs and enums")
        );
    }

    #[test]
    fn rejects_a_binder_whose_model_is_already_declared_as_a_value() {
        assert!(
            rejection_for(
                "use margaret::framework::macros::provides_route_parameter;\nuse margaret::framework::macros::route_parameter_value;\nuse margaret::framework::macros::singleton;\n#[route_parameter_value]\nstruct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("is both a #[route_parameter_value] and the model")
        );
    }

    #[test]
    fn rejects_a_value_declaration_for_a_type_that_a_binder_already_provides() {
        assert!(
            rejection_for(
                "use margaret::framework::macros::provides_route_parameter;\nuse margaret::framework::macros::route_parameter_value;\nuse margaret::framework::macros::singleton;\n#[route_parameter_value]\nstruct Zebra;\n\n#[singleton]\n#[provides_route_parameter]\nstruct AardvarkStore;\n\nimpl HttpRouteParameterBinder for AardvarkStore {\n    type Model = Zebra;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Zebra>> {}\n}\n"
            )
            .contains("is both a #[route_parameter_value] and the model")
        );
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        assert!(
            rejection_for(
                "use margaret::framework::macros::provides_route_parameter;\nuse margaret::framework::macros::singleton;\nstruct Article;\n\n#[singleton]\n#[provides_route_parameter]\nstruct First;\n\nimpl HttpRouteParameterBinder for First {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n\n#[singleton]\n#[provides_route_parameter]\nstruct Second;\n\nimpl HttpRouteParameterBinder for Second {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("has more than one route parameter binder")
        );
    }

    #[test]
    fn rejects_a_binder_that_is_not_a_struct() {
        assert!(
            rejection_for(
                "use margaret::framework::macros::provides_route_parameter;\nstruct Article;\n\n#[provides_route_parameter]\nenum ArticleStore {}\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("is only supported on structs")
        );
    }

    #[test]
    fn rejects_a_binder_that_is_not_a_singleton() {
        assert!(
            rejection_for(
                "use margaret::framework::macros::provides_route_parameter;\nstruct Article;\n\n#[provides_route_parameter]\nstruct ArticleStore;\n\nimpl HttpRouteParameterBinder for ArticleStore {\n    type Model = Article;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}\n}\n"
            )
            .contains("must also be declared as a #[singleton]")
        );
    }

    #[test]
    fn rejects_a_binder_without_a_model_that_resolves_to_a_struct() {
        assert!(
            rejection_for("use margaret::framework::macros::provides_route_parameter;\nuse margaret::framework::macros::singleton;\n#[singleton]\n#[provides_route_parameter]\nstruct Bare;\n")
                .contains("has no `type Model = <struct>` associated type")
        );
    }

    #[test]
    fn resolves_a_single_keyed_model_through_the_database() {
        assert_eq!(
            model_resolved_by(RouteModelKey::Single, &database()),
            ResolvedBy::Database("framework_database".to_string())
        );
    }

    #[test]
    fn resolves_a_single_keyed_model_without_a_database_as_unbindable() {
        assert_eq!(
            model_resolved_by(RouteModelKey::Single, &RouteDatabase::Undeclared),
            ResolvedBy::DatabaseUndeclared
        );
    }

    #[test]
    fn resolves_a_composite_keyed_model_as_unbindable() {
        assert_eq!(
            model_resolved_by(RouteModelKey::Composite, &database()),
            ResolvedBy::CompositePrimaryKey
        );
    }

    #[test]
    fn rejects_a_binder_for_a_model_bound_by_its_primary_key() {
        assert!(matches!(
            model_resolutions_for(BINDER, RouteModelKey::Single, &database()),
            Err(RequestBindingError::RouteModelWithBinder { model, binder })
                if model == "crate::Article" && binder == "crate::ArticleStore"
        ));
    }

    #[test]
    fn rejects_a_value_declaration_for_a_model_bound_by_its_primary_key() {
        assert!(matches!(
            model_resolutions_for(
                "use margaret::framework::macros::route_parameter_value;\n#[route_parameter_value]\nstruct Article;\n",
                RouteModelKey::Single,
                &database()
            ),
            Err(RequestBindingError::RouteModelDeclaredAsValue { model }) if model == "crate::Article"
        ));
    }
}
