use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::request_input_source::RequestInputSource;

fn request_input_name(source: RequestInputSource) -> &'static str {
    match source {
        RequestInputSource::Cookie => "Cookie",
        RequestInputSource::Form => "Form",
        RequestInputSource::Query => "Query",
        RequestInputSource::Json => "Json",
    }
}

pub(crate) const REQUEST_INPUTS: FrameworkVocabulary<RequestInputSource> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "http_validation",
        "request_input",
        "RequestInput",
    ],
    name: request_input_name,
    variants: &[
        RequestInputSource::Cookie,
        RequestInputSource::Form,
        RequestInputSource::Query,
        RequestInputSource::Json,
    ],
};

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::REQUEST_INPUTS;
    use crate::request_input_source::RequestInputSource;

    #[test]
    fn recognises_every_request_input_variant_by_its_canonical_path() {
        let recognised: Vec<Option<RequestInputSource>> = ["Cookie", "Form", "Query", "Json"]
            .into_iter()
            .map(|variant| {
                REQUEST_INPUTS.variant(&CanonicalPath::new(
                    [
                        "margaret",
                        "framework",
                        "http_validation",
                        "request_input",
                        "RequestInput",
                        variant,
                    ]
                    .map(ToString::to_string)
                    .to_vec(),
                ))
            })
            .collect();

        assert_eq!(
            recognised,
            vec![
                Some(RequestInputSource::Cookie),
                Some(RequestInputSource::Form),
                Some(RequestInputSource::Query),
                Some(RequestInputSource::Json),
            ]
        );
    }
}
