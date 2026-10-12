use mime::Mime;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MediaTypeClass {
    Json,
    MultipartFormData,
    Other,
    UrlEncodedForm,
}

impl MediaTypeClass {
    pub(crate) fn of(media_type: &Mime) -> Self {
        let subtype = media_type.subtype();

        match media_type.type_() {
            mime::APPLICATION if subtype == mime::JSON => Self::Json,
            mime::APPLICATION if subtype == mime::WWW_FORM_URLENCODED => Self::UrlEncodedForm,
            mime::MULTIPART if subtype == mime::FORM_DATA => Self::MultipartFormData,
            _ => Self::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use mime::Mime;

    use super::MediaTypeClass;

    fn class_of(media_type: &str) -> MediaTypeClass {
        MediaTypeClass::of(&media_type.parse::<Mime>().expect("a valid media type"))
    }

    #[test]
    fn classifies_json_regardless_of_parameters_so_a_charset_never_hides_a_json_body() {
        assert_eq!(
            class_of("application/json; charset=utf-8"),
            MediaTypeClass::Json
        );
    }

    #[test]
    fn classifies_an_urlencoded_form_so_its_body_is_read_as_urlencoded_fields() {
        assert_eq!(
            class_of("application/x-www-form-urlencoded"),
            MediaTypeClass::UrlEncodedForm
        );
    }

    #[test]
    fn classifies_multipart_form_data_despite_its_boundary_so_the_body_is_read_as_parts() {
        assert_eq!(
            class_of("multipart/form-data; boundary=X"),
            MediaTypeClass::MultipartFormData
        );
    }

    #[test]
    fn classifies_any_other_media_type_as_other() {
        assert_eq!(class_of("text/plain"), MediaTypeClass::Other);
    }
}
