use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HttpsUrlRejection {
    Malformed(url::ParseError),
    NotHttps { scheme: String },
}

impl Display for HttpsUrlRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Malformed(source) => write!(formatter, "the url is malformed: {source}"),
            Self::NotHttps { scheme } => write!(
                formatter,
                "the url uses the '{scheme}' scheme instead of https"
            ),
        }
    }
}
