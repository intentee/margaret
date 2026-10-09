use crate::sql_parameter::SqlParameter;

pub struct Statement {
    pub parameters: Vec<SqlParameter>,
    pub text: String,
}
