use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;
use margaret_registered_claims::audience::Audience;

use crate::declared_session_cookies::DeclaredSessionCookies;

pub struct IssuedSessionsDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub audience: Audience,
    pub cookies: DeclaredSessionCookies,
    pub issuer: Tag,
}
