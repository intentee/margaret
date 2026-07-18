use cookie::Cookie;

pub(crate) enum CookieOperation {
    Remove,
    Set { cookie: Cookie<'static> },
}
