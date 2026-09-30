use margaret_http::response::Response;

pub enum UserinfoAnswer {
    Answered(Response),
    ClaimsNotAnObject,
    CollidingSubject,
}
