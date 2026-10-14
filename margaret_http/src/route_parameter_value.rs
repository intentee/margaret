use crate::requirement::Requirement;
use crate::response::Response;

#[must_use]
pub fn route_parameter_value<TValue>(value: String) -> Requirement<TValue>
where
    TValue: TryFrom<String>,
{
    match TValue::try_from(value) {
        Ok(converted) => Requirement::Met(converted),
        Err(_rejected) => Requirement::Unmet(Response::not_found()),
    }
}

#[cfg(test)]
mod tests {
    use super::route_parameter_value;
    use crate::requirement::Requirement;

    struct PublishedYear(u16);

    impl TryFrom<String> for PublishedYear {
        type Error = std::num::ParseIntError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse().map(Self)
        }
    }

    #[test]
    fn converts_a_route_parameter_into_its_value_type() {
        assert!(matches!(
            route_parameter_value::<PublishedYear>("2024".to_string()),
            Requirement::Met(PublishedYear(year)) if year == 2024
        ));
    }

    #[test]
    fn answers_a_route_parameter_its_value_type_rejects_with_not_found() {
        assert!(matches!(
            route_parameter_value::<PublishedYear>("draft".to_string()),
            Requirement::Unmet(response) if response.status() == 404
        ));
    }
}
