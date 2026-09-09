use std::collections::HashMap;
use std::sync::LazyLock;

use bytes::Bytes;
use serde_json::Value;

use margaret_http_uploaded_file::uploaded_file::UploadedFile;

static NO_BYTES: LazyLock<Bytes> = LazyLock::new(Bytes::new);
static NO_FILES: LazyLock<HashMap<String, UploadedFile>> = LazyLock::new(HashMap::new);
static NO_FORM: LazyLock<HashMap<String, String>> = LazyLock::new(HashMap::new);

pub enum RequestBodyInputs {
    Collected(Bytes),
    Empty,
    Json(Value),
    Multipart {
        files: HashMap<String, UploadedFile>,
        form: HashMap<String, String>,
    },
    UrlEncoded(HashMap<String, String>),
}

impl RequestBodyInputs {
    #[must_use]
    pub fn collected(&self) -> &Bytes {
        match self {
            Self::Collected(bytes) => bytes,
            Self::Empty | Self::Json(_) | Self::Multipart { .. } | Self::UrlEncoded(_) => &NO_BYTES,
        }
    }

    #[must_use]
    pub fn files(&self) -> &HashMap<String, UploadedFile> {
        match self {
            Self::Multipart { files, .. } => files,
            Self::Collected(_) | Self::Empty | Self::Json(_) | Self::UrlEncoded(_) => &NO_FILES,
        }
    }

    #[must_use]
    pub fn form(&self) -> &HashMap<String, String> {
        match self {
            Self::Multipart { form, .. } | Self::UrlEncoded(form) => form,
            Self::Collected(_) | Self::Empty | Self::Json(_) => &NO_FORM,
        }
    }

    #[must_use]
    pub fn json(&self) -> Option<&Value> {
        match self {
            Self::Json(value) => Some(value),
            Self::Collected(_) | Self::Empty | Self::Multipart { .. } | Self::UrlEncoded(_) => None,
        }
    }
}
