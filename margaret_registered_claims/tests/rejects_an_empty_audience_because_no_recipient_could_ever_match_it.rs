use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_parsing::AudienceParsing;

#[test]
fn rejects_an_empty_audience_because_no_recipient_could_ever_match_it() {
    assert_eq!(Audience::parse(""), AudienceParsing::Empty);
}
