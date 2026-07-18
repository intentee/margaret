use crate::cookie_jar::CookieJar;

#[derive(Debug)]
pub enum RequestCookies {
    Absent,
    Present { cookie_jar: CookieJar },
}
