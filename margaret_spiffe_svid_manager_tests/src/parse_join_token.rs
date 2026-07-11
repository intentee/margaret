use anyhow::Result;
use anyhow::anyhow;

pub fn parse_join_token(output: &[u8]) -> Result<String> {
    let text = std::str::from_utf8(output)?;

    for line in text.lines() {
        if let Some(rest) = line.trim().strip_prefix("Token:") {
            return Ok(rest.trim().to_string());
        }
    }

    Err(anyhow!(
        "could not parse join token from spire-server output: {text}"
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_join_token;

    #[test]
    fn extracts_token_from_output() {
        let output = b"Token: abc123def456\n";

        let result = parse_join_token(output).unwrap();

        assert_eq!(result, "abc123def456");
    }

    #[test]
    fn errors_when_token_line_is_missing() {
        let output = b"some other output\nno token here\n";

        let result = parse_join_token(output);

        assert!(result.is_err());
    }

    #[test]
    fn errors_when_output_is_invalid_utf8() {
        let output: &[u8] = &[0xff, 0xfe, 0xfd];

        let result = parse_join_token(output);

        assert!(result.is_err());
    }
}
