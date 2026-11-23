use crate::{
    bridge::{FromKDL, ToKDL},
    types::datetime::TrayDateTime,
};
use club_kdl::{KdlDeserialize, KdlSerialize};

#[derive(Debug, KdlDeserialize, Eq, PartialEq, KdlSerialize)]
#[kdl(name = "clock")]
pub struct Clock {
    #[kdl(property)]
    pub start: TrayDateTime,
    #[kdl(property)]
    pub end: Option<TrayDateTime>,
}

impl FromKDL<'_> for Clock {}
impl ToKDL for Clock {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"clock start="2026-10-15""#;
        let actual = Clock::from_kdl(input).unwrap();
        let expected = Clock {
            start: TrayDateTime::from_str("2026-10-15").unwrap(),
            end: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r"";
        let actual = Clock::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_start_property() {
        let input = r"clock";
        let actual = Clock::from_kdl(input).unwrap_err();
        let expected_message = "in Clock: missing required field: start";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_start_property() {
        let input = r#"clock start="10""#;
        let actual = Clock::from_kdl(input).unwrap_err();
        let expected_message = "in Clock: parse error: Failed to parse DateTime";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn write_simple_node() {
        let input = r#"clock start="2026-10-15""#;
        let intermediary = Clock::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }

    #[test]
    fn write_end_node() {
        let input = r#"clock start="2026-10-15 12:50" end="2026-10-15 12:53""#;
        let intermediary = Clock::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }
}
