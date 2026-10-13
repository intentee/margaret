use crate::served_sign_in::ServedSignIn;

pub enum SignInService<'declarations> {
    Served(ServedSignIn<'declarations>),
    Unavailable,
}
