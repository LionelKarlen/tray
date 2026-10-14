use crate::{bridge::FromKDL, types::datetime::TrayDateTime};
use club_kdl::KdlDeserialize;

#[derive(Debug, KdlDeserialize, Eq, PartialEq)]
#[kdl(name = "deadline")]
pub struct Deadline {
    #[kdl(argument)]
    pub on: TrayDateTime,
}

impl FromKDL<'_> for Deadline {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"deadline "2026-10-15""#;
        let actual = Deadline::from_kdl(input).unwrap();
        let expected = Deadline {
            on: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r#""#;
        let actual = Deadline::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_argument() {
        let input = r#"deadline"#;
        let actual = Deadline::from_kdl(input).unwrap_err();
        let expected_message = "in Deadline: missing required argument at index 0";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_argument() {
        let input = r#"deadline "10""#;
        let actual = Deadline::from_kdl(input).unwrap_err();
        let expected_message = "in Deadline: parse error: Failed to parse DateTime";
        assert_eq!(actual.to_string(), expected_message);
    }
}
