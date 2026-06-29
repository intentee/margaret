use syn::ReturnType;
use syn::Signature;
use syn::Type;

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

fn constructor_returns_self(signature: &Signature) -> bool {
    match &signature.output {
        ReturnType::Type(_, return_type) => is_self_type(return_type),
        ReturnType::Default => false,
    }
}

fn is_self_type(return_type: &Type) -> bool {
    match return_type {
        Type::Path(type_path) => type_path.path.is_ident("Self"),
        _ => false,
    }
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
        Some(method) if constructor_returns_self(method.signature()) => {
            Ok(ConstructionSource::Constructor(method))
        }
        Some(_) => Err(ContainerError::ConstructorReturnTypeMismatch {
            singleton: concrete_path.to_string(),
        }),
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

#[cfg(test)]
mod tests {
    use syn::ImplItemFn;
    use syn::Signature;

    use super::constructor_returns_self;

    fn signature(source: &str) -> Signature {
        let method: ImplItemFn = syn::parse_str(source).expect("a method fixture parses");

        method.sig
    }

    #[test]
    fn accepts_self_return() {
        assert!(constructor_returns_self(&signature("fn new() -> Self {}")));
    }

    #[test]
    fn rejects_concrete_return() {
        assert!(!constructor_returns_self(&signature(
            "fn new() -> Config {}"
        )));
    }

    #[test]
    fn rejects_non_path_return() {
        assert!(!constructor_returns_self(&signature("fn new() -> () {}")));
    }

    #[test]
    fn rejects_missing_return() {
        assert!(!constructor_returns_self(&signature("fn new() {}")));
    }
}
