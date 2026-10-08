use crate::completeness::Completeness;

#[derive(Debug, Eq, PartialEq)]
pub struct Children<Child> {
    pub completeness: Completeness,
    pub records: Vec<Child>,
}
