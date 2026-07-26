use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::ReturnType;
use syn::Signature;
use syn::Type;

fn is_self_type(candidate: &Type) -> bool {
    matches!(candidate, Type::Path(type_path) if type_path.path.is_ident("Self"))
}

fn result_ok_is_self(segment: &PathSegment) -> bool {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };

    matches!(
        arguments.args.first(),
        Some(GenericArgument::Type(ok_type)) if is_self_type(ok_type)
    )
}

fn returns_result_of_self(return_type: &Type) -> bool {
    let Type::Path(type_path) = return_type else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Result" && result_ok_is_self(segment))
}

pub(crate) enum ConstructorReturn {
    Fallible,
    Infallible,
}

impl ConstructorReturn {
    pub(crate) fn classify(signature: &Signature) -> Option<Self> {
        let ReturnType::Type(_, return_type) = &signature.output else {
            return None;
        };

        if is_self_type(return_type) {
            Some(Self::Infallible)
        } else if returns_result_of_self(return_type) {
            Some(Self::Fallible)
        } else {
            None
        }
    }

    pub(crate) fn is_fallible(&self) -> bool {
        matches!(self, ConstructorReturn::Fallible)
    }
}

#[cfg(test)]
mod tests {
    use syn::ImplItemFn;
    use syn::Signature;

    use super::ConstructorReturn;

    fn signature(source: &str) -> Signature {
        let method: ImplItemFn = syn::parse_str(source).expect("a method fixture parses");

        method.sig
    }

    fn classify(source: &str) -> Option<ConstructorReturn> {
        ConstructorReturn::classify(&signature(source))
    }

    #[test]
    fn classifies_a_self_return_as_infallible() {
        assert!(
            !classify("fn new() -> Self {}")
                .expect("the return type classifies")
                .is_fallible()
        );
    }

    #[test]
    fn classifies_an_anyhow_result_of_self_as_fallible() {
        assert!(
            classify("fn new() -> anyhow::Result<Self> {}")
                .expect("the return type classifies")
                .is_fallible()
        );
    }

    #[test]
    fn classifies_a_two_argument_result_of_self_as_fallible() {
        assert!(
            classify("fn new() -> Result<Self, std::io::Error> {}")
                .expect("the return type classifies")
                .is_fallible()
        );
    }

    #[test]
    fn rejects_a_concrete_return() {
        assert!(classify("fn new() -> Config {}").is_none());
    }

    #[test]
    fn rejects_a_result_whose_success_type_is_not_self() {
        assert!(classify("fn new() -> Result<Config, Self> {}").is_none());
    }

    #[test]
    fn rejects_a_result_without_type_arguments() {
        assert!(classify("fn new() -> Result {}").is_none());
    }

    #[test]
    fn rejects_a_non_path_return() {
        assert!(classify("fn new() -> () {}").is_none());
    }

    #[test]
    fn rejects_a_missing_return() {
        assert!(classify("fn new() {}").is_none());
    }
}
