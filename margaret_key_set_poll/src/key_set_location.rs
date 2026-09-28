use url::Url;

pub enum KeySetLocation<TFailure> {
    Cancelled,
    Failed(TFailure),
    Located(Url),
}
