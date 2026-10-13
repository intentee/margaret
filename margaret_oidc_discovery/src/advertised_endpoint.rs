use url::Url;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdvertisedEndpoint {
    Advertised(Url),
    Unadvertised,
}
