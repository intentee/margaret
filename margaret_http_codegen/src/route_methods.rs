use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_route_method::route_method::RouteMethod;

fn route_method_name(method: RouteMethod) -> &'static str {
    match method {
        RouteMethod::Delete => "Delete",
        RouteMethod::Get => "Get",
        RouteMethod::Patch => "Patch",
        RouteMethod::Post => "Post",
        RouteMethod::Put => "Put",
        RouteMethod::Query => "Query",
    }
}

pub(crate) const ROUTE_METHODS: FrameworkVocabulary<RouteMethod> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "route_method",
        "route_method",
        "RouteMethod",
    ],
    name: route_method_name,
    variants: &[
        RouteMethod::Delete,
        RouteMethod::Get,
        RouteMethod::Patch,
        RouteMethod::Post,
        RouteMethod::Put,
        RouteMethod::Query,
    ],
};

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_route_method::route_method::RouteMethod;

    use super::ROUTE_METHODS;

    #[test]
    fn reads_back_every_method_it_renders() {
        let methods = [
            RouteMethod::Delete,
            RouteMethod::Get,
            RouteMethod::Patch,
            RouteMethod::Post,
            RouteMethod::Put,
            RouteMethod::Query,
        ];
        let read_back: Vec<Option<RouteMethod>> = methods
            .into_iter()
            .map(|method| {
                ROUTE_METHODS.variant(&CanonicalPath::new(
                    ROUTE_METHODS
                        .tokens(method)
                        .into_iter()
                        .filter_map(|token| match token {
                            proc_macro2::TokenTree::Ident(identifier) => {
                                Some(identifier.to_string())
                            }
                            _ => None,
                        })
                        .collect(),
                ))
            })
            .collect();

        assert_eq!(read_back, methods.map(Some).to_vec());
    }
}
