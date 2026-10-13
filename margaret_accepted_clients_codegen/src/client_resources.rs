use crate::admitted_resource::AdmittedResource;

#[derive(Debug)]
pub enum ClientResources {
    Admitted(Vec<AdmittedResource>),
    Withheld,
}

impl ClientResources {
    #[must_use]
    pub fn admitted(&self) -> &[AdmittedResource] {
        match self {
            Self::Admitted(resources) => resources,
            Self::Withheld => &[],
        }
    }
}
