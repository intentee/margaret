use margaret_attributes::canonical_path::CanonicalPath;
use margaret_http_codegen::framework_input::FrameworkInput;
use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
use margaret_sessions_codegen::sessions_item::SessionsItem;
use margaret_sessions_codegen::sessions_item_path::sessions_item_path;

pub enum ServedSessionEndpoint {
    Refresh,
    SignOut { landing: RouteUrlInput },
}

impl ServedSessionEndpoint {
    #[must_use]
    pub fn handler_path(&self) -> CanonicalPath {
        sessions_item_path(self.item())
    }

    #[must_use]
    pub fn input(&self) -> FrameworkInput {
        match self {
            Self::Refresh => FrameworkInput::Content,
            Self::SignOut { .. } => FrameworkInput::Head,
        }
    }

    #[must_use]
    pub fn item(&self) -> SessionsItem {
        match self {
            Self::Refresh => SessionsItem::SessionRefreshEndpoint,
            Self::SignOut { .. } => SessionsItem::SessionSignOutEndpoint,
        }
    }
}
