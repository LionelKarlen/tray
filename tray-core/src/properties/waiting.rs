use crate::{bridge::FromKDL, types::datetime::TrayDateTime};
use club_kdl::KdlDeserialize;

#[derive(Debug, KdlDeserialize, Eq, PartialEq)]
#[kdl(name = "waiting")]
pub struct Waiting {
    #[kdl(property)]
    pub since: TrayDateTime,
}

impl FromKDL<'_> for Waiting {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"waiting since="2026-10-15""#;
        let actual = Waiting::from_kdl(input).unwrap();
        let expected = Waiting {
            since: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r"";
        let actual = Waiting::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_since_property() {
        let input = r"waiting";
        let actual = Waiting::from_kdl(input).unwrap_err();
        let expected_message = "in Waiting: missing required field: since";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_since_property() {
        let input = r#"waiting since="10""#;
        let actual = Waiting::from_kdl(input).unwrap_err();
        let expected_message = "in Waiting: parse error: Failed to parse DateTime";
        assert_eq!(actual.to_string(), expected_message);
    }
}
