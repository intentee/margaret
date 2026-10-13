use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;

pub struct ConsumedSessionsDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub cookie_domain_from: EnvironmentVariableName,
    pub issuer: Tag,
    pub refresh_url_from: EnvironmentVariableName,
}
