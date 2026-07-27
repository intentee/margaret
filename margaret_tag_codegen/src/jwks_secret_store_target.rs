use margaret_attributes::tag::Tag;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JwksSecretStoreTarget {
    Client(Tag),
    Server,
}
