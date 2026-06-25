use std::future::Future;

use crate::crud_action::CrudAction;
use crate::request::Request;

pub trait CrudActionGate {
    type Subject;

    fn can(
        &self,
        request: &Request,
        subject: &Self::Subject,
        action: CrudAction,
    ) -> impl Future<Output = bool> + Send;
}
