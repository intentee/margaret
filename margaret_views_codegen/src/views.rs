use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_query::AttributeQuery;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::view::View;
use crate::view_arguments::ViewArguments;
use crate::views_codegen_error::ViewsCodegenError;

pub(crate) fn views(index: &AttributeIndex) -> Result<Vec<View>, ViewsCodegenError> {
    let selector = AttributeSelector::from_marker("renders_view");
    let singleton_selector = AttributeSelector::from_marker("singleton");
    let mut resolved: Vec<View> = Vec::new();
    let mut seen_views: HashSet<String> = HashSet::new();
    let mut seen_names: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let view = item.canonical_path().to_string();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(ViewsCodegenError::ViewNotAStruct { view });
        };

        if !seen_views.insert(view.clone()) {
            return Err(ViewsCodegenError::DuplicateViewDeclaration { view });
        }

        let singletons = AttributeQuery::new(item).find_all(&singleton_selector);
        if let Some(singleton) = singletons.first()
            && singleton.args()?.path("provides")?.is_some()
        {
            return Err(ViewsCodegenError::ViewProvidesInterface { view });
        }

        let ViewArguments { name } = ViewArguments::parse(matched.args()?, &view)?;

        if !is_snake_case_identifier(&name) {
            return Err(ViewsCodegenError::InvalidViewName { view, name });
        }

        if let Some(first) = seen_names.get(&name) {
            return Err(ViewsCodegenError::DuplicateViewName {
                name,
                first: first.clone(),
                second: view,
            });
        }

        seen_names.insert(name.clone(), view);

        resolved.push(View {
            accessor: identifier.field().to_string(),
            concrete_path: item.canonical_path().clone(),
            name,
        });
    }

    resolved.sort_by(|first, second| first.name.cmp(&second.name));

    Ok(resolved)
}
