use crate::jwt_expectation::JwtExpectation;

pub trait JwtAddressee {
    fn jwt_expectation(&self) -> JwtExpectation<'_>;
}
