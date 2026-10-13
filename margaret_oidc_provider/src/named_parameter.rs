pub(crate) struct NamedParameter<'value> {
    pub(crate) name: &'static str,
    pub(crate) value: Option<&'value str>,
}
