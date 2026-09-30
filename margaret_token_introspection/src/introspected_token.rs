use oauth2::ClientId;
use oauth2::Scope;

pub struct IntrospectedToken<TClaims> {
    pub claims: TClaims,
    pub client_id: Option<ClientId>,
    pub scopes: Option<Vec<Scope>>,
    pub subject: Option<String>,
    pub username: Option<String>,
}
