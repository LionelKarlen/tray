use crate::{
    bridge::{FromKDL, ToKDL},
    types::datetime::TrayDateTime,
};
use club_kdl::{KdlDeserialize, KdlSerialize};

#[derive(Debug, KdlDeserialize, Eq, PartialEq, KdlSerialize)]
#[kdl(name = "sleeping")]
pub struct Sleeping {
    #[kdl(property)]
    pub until: TrayDateTime,
}

impl FromKDL<'_> for Sleeping {}
impl ToKDL for Sleeping {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"sleeping until="2026-10-15""#;
        let actual = Sleeping::from_kdl(input).unwrap();
        let expected = Sleeping {
            until: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r"";
        let actual = Sleeping::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_until_property() {
        let input = r"sleeping";
        let actual = Sleeping::from_kdl(input).unwrap_err();
        let expected_message = "in Sleeping: missing required field: until";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_until_property() {
        let input = r#"sleeping until="10""#;
        let actual = Sleeping::from_kdl(input).unwrap_err();
        let expected_message = "in Sleeping: parse error: Failed to parse DateTime";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn write_simple_node() {
        let input = r#"sleeping until="2026-10-15""#;
        let intermediary = Sleeping::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }
}
