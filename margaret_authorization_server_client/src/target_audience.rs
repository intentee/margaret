use url::Url;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TargetAudience {
    Audience(String),
    Resource(Url),
    Unspecified,
}
