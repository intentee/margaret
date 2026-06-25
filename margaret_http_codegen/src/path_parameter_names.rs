use matchit::InsertError;
use matchit::Router;

pub(crate) fn path_parameter_names(pattern: &str) -> Result<Vec<String>, InsertError> {
    let mut router: Router<()> = Router::new();

    router.insert(pattern, ())?;

    let matched = router
        .at(pattern)
        .expect("a freshly inserted route matches its own pattern");

    Ok(matched
        .params
        .iter()
        .map(|(name, _value)| name.to_string())
        .collect())
}
