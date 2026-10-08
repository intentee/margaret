use margaret_attributes::indexed_attribute::IndexedAttribute;

use crate::bearer_token_addressee::BearerTokenAddressee;

pub struct ScannedBearerToken<'index> {
    pub addressee: BearerTokenAddressee,
    pub attribute: &'index IndexedAttribute,
}
