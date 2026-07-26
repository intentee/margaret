use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::struct_shape::StructShape;

use crate::construction_source::ConstructionSource;
use crate::container_error::ContainerError;

fn has_constructor_attribute(method: &IndexedMethod) -> bool {
    method.attributes().iter().any(|attribute| {
        attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "constructor")
    })
}

pub(crate) fn resolve_construction<'index>(
    methods: &'index [IndexedMethod],
    concrete_path: &CanonicalPath,
    shape: StructShape,
) -> Result<ConstructionSource<'index>, ContainerError> {
    let mut found: Vec<&IndexedMethod> = methods
        .iter()
        .filter(|method| has_constructor_attribute(method))
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

    match found.pop() {
        Some(method) => Ok(ConstructionSource::Constructor(method)),
        None => {
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
}
