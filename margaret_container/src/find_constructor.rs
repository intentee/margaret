use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use syn::ReturnType;
use syn::Signature;
use syn::Type;

use crate::container_error::ContainerError;

pub(crate) fn find_constructor<'index>(
    holders: &'index [AttributeHolder],
    concrete_path: &CanonicalPath,
) -> Result<&'index IndexedMethod, ContainerError> {
    let mut found: Vec<&IndexedMethod> = holders
        .iter()
        .filter_map(|holder| match holder {
            AttributeHolder::Method(method) => Some(method),
            AttributeHolder::Item(_) => None,
        })
        .filter(|method| {
            method.self_type_path() == concrete_path && has_constructor_attribute(method)
        })
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
        Some(method) if constructor_returns_self(method.signature()) => Ok(method),
        Some(_) => Err(ContainerError::ConstructorReturnTypeMismatch {
            singleton: concrete_path.to_string(),
        }),
        None => Err(ContainerError::MissingConstructor {
            singleton: concrete_path.to_string(),
        }),
    }
}

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
