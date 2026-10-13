use headers::Authorization;
use headers::authorization::Basic;

pub enum PresentedClientAuthentication {
    Assertion(String),
    Basic(Authorization<Basic>),
}
