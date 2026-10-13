use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct LibraryProfile {
    founded: Option<u16>,
    name: String,
}

impl LibraryProfile {
    #[must_use]
    #[constructor]
    pub fn create(
        #[environment_variable(from = "MARGARET_COVERAGE_FIXTURE_LIBRARY_NAME")] name: String,
        #[environment_variable(from = "MARGARET_COVERAGE_FIXTURE_LIBRARY_FOUNDED")] founded: Option<
            u16,
        >,
    ) -> Self {
        Self { founded, name }
    }

    #[must_use]
    pub fn description(&self) -> String {
        match self.founded {
            Some(founded) => format!("{} (founded {founded})", self.name),
            None => self.name.clone(),
        }
    }
}
