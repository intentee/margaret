use crate::http_route_symbol::HttpRouteSymbol;

pub struct Forward {
    key: &'static str,
}

impl Forward {
    pub fn to(symbol: impl HttpRouteSymbol) -> Self {
        Self {
            key: symbol.route_key(),
        }
    }

    pub(crate) fn key(&self) -> &'static str {
        self.key
    }
}

#[cfg(test)]
mod tests {
    use super::Forward;
    use crate::http_route_symbol::HttpRouteSymbol;

    struct GetLogin;

    impl HttpRouteSymbol for GetLogin {
        fn route_key(&self) -> &'static str {
            "crate::routes::get_login::GetLogin"
        }
    }

    #[test]
    fn carries_the_target_route_key() {
        assert_eq!(
            Forward::to(GetLogin).key(),
            "crate::routes::get_login::GetLogin"
        );
    }
}
