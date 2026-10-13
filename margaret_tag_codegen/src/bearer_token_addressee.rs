use margaret_attributes::tag::Tag;

pub enum BearerTokenAddressee {
    Client(Tag),
    Issuer(Tag),
    Resource(Tag),
}
