pub struct Forward {
    name: &'static str,
}

impl Forward {
    pub fn to(name: &'static str) -> Self {
        Self { name }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::Forward;

    #[test]
    fn carries_the_target_route_name() {
        assert_eq!(
            Forward::to("crate::routes::get_login::GetLogin").name(),
            "crate::routes::get_login::GetLogin"
        );
    }
}
