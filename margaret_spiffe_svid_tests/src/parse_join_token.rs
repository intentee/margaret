use anyhow::Result;

use crate::join_token_output::JoinTokenOutput;

pub fn parse_join_token(output: &[u8]) -> Result<String> {
    let JoinTokenOutput { value } = serde_json::from_slice(output)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::parse_join_token;

    #[test]
    fn extracts_the_token_value_from_the_json_output() {
        let output = br#"{"expires_at":"1784066656","value":"abc123def456"}"#;

        let token = parse_join_token(output).expect("the join token is parsed");

        assert_eq!(token, "abc123def456");
    }

    #[test]
    fn errors_when_the_output_carries_no_token_value() {
        let output = br#"{"expires_at":"1784066656"}"#;

        assert!(parse_join_token(output).is_err());
    }
}
