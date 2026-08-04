use crate::project_name::ProjectName;
use crate::project_name_rejection::ProjectNameRejection;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ProjectNameOutcome {
    Accepted(ProjectName),
    Rejected(ProjectNameRejection),
}
