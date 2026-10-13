use crate::served_authorization::ServedAuthorization;

#[derive(Debug, Eq, PartialEq)]
pub enum DerivedAuthorization {
    Served(ServedAuthorization),
    Unserved,
}
