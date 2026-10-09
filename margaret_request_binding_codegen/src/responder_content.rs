use crate::bound_parameter::BoundParameter;
use crate::content_binding::ContentBinding;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;

fn demanded_content(BoundParameter { binding, holder }: &BoundParameter) -> Option<ContentBinding> {
    match binding {
        RequestBinding::FormContent { .. } => Some(ContentBinding::FormFields),
        RequestBinding::JsonContent { .. } => Some(ContentBinding::Json),
        RequestBinding::RequestBodyStream => Some(ContentBinding::Stream {
            stream: holder.clone(),
        }),
        RequestBinding::UploadedFiles => Some(ContentBinding::MultipartFiles {
            files: holder.clone(),
        }),
        RequestBinding::AssetBag
        | RequestBinding::AuthenticatedUser { .. }
        | RequestBinding::BearerToken { .. }
        | RequestBinding::IntrospectedBearerToken { .. }
        | RequestBinding::BoundRouteParameter { .. }
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Injectable { .. }
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Session { .. }
        | RequestBinding::Views => None,
    }
}

fn merged_content(read: ContentBinding, demanded: ContentBinding) -> Option<ContentBinding> {
    match read {
        ContentBinding::FormFields => match demanded {
            ContentBinding::FormFields => Some(ContentBinding::FormFields),
            ContentBinding::MultipartFiles { files } => {
                Some(ContentBinding::MultipartFieldsAndFiles { files })
            }
            ContentBinding::Json
            | ContentBinding::MultipartFieldsAndFiles { .. }
            | ContentBinding::Stream { .. } => None,
        },
        ContentBinding::Json => match demanded {
            ContentBinding::Json => Some(ContentBinding::Json),
            ContentBinding::FormFields
            | ContentBinding::MultipartFieldsAndFiles { .. }
            | ContentBinding::MultipartFiles { .. }
            | ContentBinding::Stream { .. } => None,
        },
        ContentBinding::MultipartFieldsAndFiles { files }
        | ContentBinding::MultipartFiles { files } => match demanded {
            ContentBinding::FormFields => Some(ContentBinding::MultipartFieldsAndFiles { files }),
            ContentBinding::Json
            | ContentBinding::MultipartFieldsAndFiles { .. }
            | ContentBinding::MultipartFiles { .. }
            | ContentBinding::Stream { .. } => None,
        },
        ContentBinding::Stream { .. } => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponderContent {
    Read(ContentBinding),
    Unread,
}

impl ResponderContent {
    /// # Errors
    ///
    /// Returns `RequestBindingError::ConflictingContentBindings` when the parameters read the
    /// request body in more than one way.
    pub fn of(parameters: &[BoundParameter], subject: &str) -> Result<Self, RequestBindingError> {
        let mut content = Self::Unread;

        for parameter in parameters {
            let Some(demanded) = demanded_content(parameter) else {
                continue;
            };

            content = match content {
                Self::Unread => Self::Read(demanded),
                Self::Read(read) => {
                    Self::Read(merged_content(read, demanded).ok_or_else(|| {
                        RequestBindingError::ConflictingContentBindings {
                            subject: subject.to_string(),
                        }
                    })?)
                }
            };
        }

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use super::ResponderContent;
    use crate::bound_parameter::BoundParameter;
    use crate::content_binding::ContentBinding;
    use crate::form_request_extraction::FormRequestExtraction;
    use crate::request_binding::RequestBinding;
    use crate::request_binding_error::RequestBindingError;

    fn parameter(binding: RequestBinding) -> BoundParameter {
        BoundParameter {
            binding,
            holder: format_ident!("argument"),
        }
    }

    fn files() -> BoundParameter {
        BoundParameter {
            binding: RequestBinding::UploadedFiles,
            holder: format_ident!("attachments"),
        }
    }

    fn form() -> BoundParameter {
        parameter(RequestBinding::FormContent {
            extraction: FormRequestExtraction::Model,
        })
    }

    fn json() -> BoundParameter {
        parameter(RequestBinding::JsonContent {
            extraction: FormRequestExtraction::Model,
        })
    }

    fn content_of(parameters: &[BoundParameter]) -> Result<ResponderContent, RequestBindingError> {
        ResponderContent::of(parameters, "responder 'crate::Upload'")
    }

    fn assert_reads(parameters: &[BoundParameter], expected: &ContentBinding) {
        assert!(matches!(
            content_of(parameters),
            Ok(ResponderContent::Read(binding)) if binding == *expected
        ));
    }

    fn assert_conflicts(parameters: &[BoundParameter]) {
        assert!(matches!(
            content_of(parameters),
            Err(RequestBindingError::ConflictingContentBindings { subject })
                if subject == "responder 'crate::Upload'"
        ));
    }

    #[test]
    fn leaves_the_body_unread_without_a_content_parameter() {
        assert_eq!(
            content_of(&[parameter(RequestBinding::CurrentRequest)])
                .expect("head parameters never conflict"),
            ResponderContent::Unread
        );
    }

    #[test]
    fn reads_several_form_models_from_one_set_of_fields() {
        assert_reads(&[form(), form()], &ContentBinding::FormFields);
    }

    #[test]
    fn reads_several_json_models_from_one_value() {
        assert_reads(&[json(), json()], &ContentBinding::Json);
    }

    #[test]
    fn reads_uploaded_files_alone() {
        assert_reads(
            &[files()],
            &ContentBinding::MultipartFiles {
                files: format_ident!("attachments"),
            },
        );
    }

    #[test]
    fn reads_form_fields_declared_before_the_uploaded_files_from_one_multipart_body() {
        assert_reads(
            &[form(), files()],
            &ContentBinding::MultipartFieldsAndFiles {
                files: format_ident!("attachments"),
            },
        );
    }

    #[test]
    fn reads_form_fields_declared_after_the_uploaded_files_from_one_multipart_body() {
        assert_reads(
            &[files(), form(), form()],
            &ContentBinding::MultipartFieldsAndFiles {
                files: format_ident!("attachments"),
            },
        );
    }

    #[test]
    fn reads_a_stream_alone() {
        assert_reads(
            &[parameter(RequestBinding::RequestBodyStream)],
            &ContentBinding::Stream {
                stream: format_ident!("argument"),
            },
        );
    }

    #[test]
    fn rejects_form_fields_and_json_together() {
        assert_conflicts(&[form(), json()]);
    }

    #[test]
    fn rejects_json_and_form_fields_together() {
        assert_conflicts(&[json(), form()]);
    }

    #[test]
    fn rejects_uploaded_files_bound_twice() {
        assert_conflicts(&[files(), files()]);
    }

    #[test]
    fn rejects_a_stream_together_with_another_content_parameter() {
        assert_conflicts(&[parameter(RequestBinding::RequestBodyStream), form()]);
    }
}
