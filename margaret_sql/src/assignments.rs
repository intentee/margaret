use crate::assignment::Assignment;

#[derive(Clone)]
pub struct Assignments {
    pub first: Assignment,
    pub rest: Vec<Assignment>,
}
