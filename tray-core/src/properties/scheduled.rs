use crate::{
    bridge::{FromKDL, ToKDL},
    types::datetime::TrayDateTime,
};
use club_kdl::{KdlDeserialize, KdlSerialize};

#[derive(Debug, KdlDeserialize, Eq, PartialEq, KdlSerialize)]
#[kdl(name = "scheduled")]
pub struct Scheduled {
    #[kdl(argument)]
    pub on: TrayDateTime,
}

impl FromKDL<'_> for Scheduled {}
impl ToKDL for Scheduled {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"scheduled "2026-10-15""#;
        let actual = Scheduled::from_kdl(input).unwrap();
        let expected = Scheduled {
            on: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r"";
        let actual = Scheduled::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_argument() {
        let input = r"scheduled";
        let actual = Scheduled::from_kdl(input).unwrap_err();
        let expected_message = "in Scheduled: missing required argument at index 0";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_argument() {
        let input = r#"scheduled "10""#;
        let actual = Scheduled::from_kdl(input).unwrap_err();
        let expected_message = "in Scheduled: parse error: Failed to parse DateTime";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn write_simple_node() {
        let input = r#"scheduled "2026-10-15""#;
        let intermediary = Scheduled::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }
}
