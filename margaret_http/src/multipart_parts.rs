use http_body_util::BodyDataStream;
use mime::Mime;
use multer::Multipart;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::multipart_part::MultipartPart;
use crate::multipart_rejection::multipart_rejection;
use crate::named_value::NamedValue;
use crate::request_body::RequestBody;
use crate::uploaded_part::UploadedPart;

pub(crate) struct MultipartParts {
    limit: BodyLimit,
    multipart: Multipart<'static>,
}

impl MultipartParts {
    pub(crate) fn open(
        media_type: &Mime,
        body: RequestBody,
        limit: BodyLimit,
    ) -> BodyReading<Self> {
        let Some(boundary) = media_type.get_param(mime::BOUNDARY) else {
            return BodyReading::Rejected(BodyRejection::MissingMultipartBoundary);
        };

        match body.limited(limit) {
            BodyReading::Read(limited) => BodyReading::Read(Self {
                limit,
                multipart: Multipart::new(BodyDataStream::new(limited), boundary.as_str()),
            }),
            BodyReading::Rejected(rejection) => BodyReading::Rejected(rejection),
        }
    }

    pub(crate) async fn next_part(&mut self) -> BodyReading<MultipartPart> {
        let field = match self.multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => return BodyReading::Read(MultipartPart::End),
            Err(source) => return BodyReading::Rejected(multipart_rejection(source, self.limit)),
        };
        let Some(name) = field.name().map(str::to_string) else {
            return BodyReading::Rejected(BodyRejection::NamelessMultipartField);
        };

        match field.file_name().map(str::to_string) {
            Some(file_name) => BodyReading::Read(MultipartPart::File(Box::new(UploadedPart {
                content_type: field
                    .content_type()
                    .unwrap_or(&mime::APPLICATION_OCTET_STREAM)
                    .essence_str()
                    .to_string(),
                field,
                file_name,
                limit: self.limit,
                name,
            }))),
            None => match field.text().await {
                Ok(value) => BodyReading::Read(MultipartPart::Text(NamedValue { name, value })),
                Err(source) => BodyReading::Rejected(multipart_rejection(source, self.limit)),
            },
        }
    }
}
