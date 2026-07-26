use syn::GenericArgument;
use syn::PathArguments;
use syn::ReturnType;
use syn::Signature;
use syn::Type;

fn is_self_type(candidate: &Type) -> bool {
    matches!(candidate, Type::Path(type_path) if type_path.path.is_ident("Self"))
}

fn generic_over_self(return_type: &Type) -> Option<&Type> {
    let Type::Path(type_path) = return_type else {
        return None;
    };

    type_path.path.segments.last().and_then(|segment| {
        let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            return None;
        };

        match arguments.args.first() {
            Some(GenericArgument::Type(ok_type)) if is_self_type(ok_type) => Some(return_type),
            _ => None,
        }
    })
}

pub(crate) enum ConstructorReturnShape<'signature> {
    GenericOverSelf(&'signature Type),
    SelfValue,
    Unsupported,
}

impl<'signature> ConstructorReturnShape<'signature> {
    pub(crate) fn of(signature: &'signature Signature) -> Self {
        let ReturnType::Type(_, return_type) = &signature.output else {
            return ConstructorReturnShape::Unsupported;
        };

        if is_self_type(return_type) {
            ConstructorReturnShape::SelfValue
        } else if let Some(generic) = generic_over_self(return_type) {
            ConstructorReturnShape::GenericOverSelf(generic)
        } else {
            ConstructorReturnShape::Unsupported
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::ImplItemFn;
    use syn::Signature;

    use super::ConstructorReturnShape;

    fn shape_label(source: &str) -> &'static str {
        let method: ImplItemFn = syn::parse_str(source).expect("a method fixture parses");
        let signature: Signature = method.sig;

        match ConstructorReturnShape::of(&signature) {
            ConstructorReturnShape::GenericOverSelf(_) => "generic_over_self",
            ConstructorReturnShape::SelfValue => "self",
            ConstructorReturnShape::Unsupported => "unsupported",
        }
    }

    #[test]
    fn detects_a_self_return() {
        assert_eq!(shape_label("fn new() -> Self {}"), "self");
    }

    #[test]
    fn detects_a_generic_whose_first_argument_is_self() {
        assert_eq!(
            shape_label("fn new() -> anyhow::Result<Self> {}"),
            "generic_over_self"
        );
    }

    #[test]
    fn detects_a_two_argument_generic_over_self() {
        assert_eq!(
            shape_label("fn new() -> Result<Self, std::io::Error> {}"),
            "generic_over_self"
        );
    }

    #[test]
    fn rejects_a_concrete_return() {
        assert_eq!(shape_label("fn new() -> Config {}"), "unsupported");
    }

    #[test]
    fn rejects_a_generic_whose_first_argument_is_not_self() {
        assert_eq!(
            shape_label("fn new() -> Result<Config, Self> {}"),
            "unsupported"
        );
    }

    #[test]
    fn rejects_a_generic_without_type_arguments() {
        assert_eq!(shape_label("fn new() -> Result {}"), "unsupported");
    }

    #[test]
    fn rejects_a_non_path_return() {
        assert_eq!(shape_label("fn new() -> () {}"), "unsupported");
    }

    #[test]
    fn rejects_a_missing_return() {
        assert_eq!(shape_label("fn new() {}"), "unsupported");
    }
}
