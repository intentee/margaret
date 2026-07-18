use crate::cookie_attributes::CookieAttributes;

pub(crate) enum CookieOperation {
    Remove,
    Set {
        attributes: CookieAttributes,
        value: String,
    },
}
