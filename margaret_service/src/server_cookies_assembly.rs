pub enum ServerCookiesAssembly {
    Cookied {
        cookie_domain_argument: &'static str,
        cookie_insecure_argument: &'static str,
    },
    Cookieless,
}
