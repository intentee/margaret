use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::struct_shape::StructShape;

use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;
use crate::declared_constructor_outcome::declared_constructor_outcome;

pub(crate) fn resolve_construction<'index>(
    index: &AttributeIndex,
    item: &'index IndexedItem,
    shape: StructShape,
) -> Result<ConstructionSource<'index>, ContainerError> {
    let concrete_path = item.canonical_path();
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| method.has_framework_attribute(FrameworkAttribute::Constructor))
        .collect();

    if found.len() > 1 {
        return Err(ContainerError::AmbiguousConstructor {
            singleton: concrete_path.to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    if let Some(method) = found.pop() {
        Ok(ConstructionSource::Constructor {
            method,
            outcome: declared_constructor_outcome(index, item, method),
        })
    } else {
        let field_count = match shape {
            StructShape::Unit => 0,
            StructShape::Named { field_count } | StructShape::Unnamed { field_count } => {
                field_count
            }
        };

        if field_count > 0 {
            Err(ContainerError::SingletonRequiresConstructor {
                singleton: concrete_path.to_string(),
                field_count,
            })
        } else {
            Ok(ConstructionSource::Fieldless(shape))
        }
    }
}
