use margaret_attributes::tag::Tag;

#[derive(Debug, Eq, PartialEq)]
pub enum JwksSecretStoreTarget {
    Client(Tag),
    Server,
}
