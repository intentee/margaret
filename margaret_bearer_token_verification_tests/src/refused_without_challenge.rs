use margaret_http::token_admission::TokenAdmission;

#[must_use]
pub fn refused_without_challenge<TToken>(admission: &TokenAdmission<TToken>, status: u16) -> bool {
    matches!(
        admission,
        TokenAdmission::Refused(response)
            if response.status() == status
                && matches!(
                    response.headers(),
                    [header] if header.name == "content-type"
                        && header.value == "text/plain; charset=utf-8"
                )
    )
}
