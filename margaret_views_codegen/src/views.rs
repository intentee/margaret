use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;

use crate::view::View;
use crate::view_arguments::ViewArguments;
use crate::views_codegen_error::ViewsCodegenError;

pub(crate) fn views(index: &AttributeIndex) -> Result<Vec<View>, ViewsCodegenError> {
    let mut resolved: Vec<View> = Vec::new();
    let mut seen_names: HashMap<String, String> = HashMap::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::RendersView) {
        let item = matched.item();
        let view = item.canonical_path().to_string();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(ViewsCodegenError::ViewNotAStruct { view });
        };

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
