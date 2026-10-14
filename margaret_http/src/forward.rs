use cookie::Cookie;

pub struct Forward<TTarget> {
    pub(crate) cookies: Vec<Cookie<'static>>,
    pub(crate) target: TTarget,
}

impl<TTarget> Forward<TTarget> {
    pub(crate) fn preceded_by(mut self, cookies: &[Cookie<'static>]) -> Self {
        self.cookies.splice(0..0, cookies.iter().cloned());

        self
    }
}
