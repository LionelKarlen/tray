use crate::bridge::FromKDL;
use club_kdl::KdlDeserialize;

#[derive(Debug, KdlDeserialize, Eq, PartialEq)]
#[kdl(name = "blocked")]
pub struct Blocked {
    #[kdl(argument)]
    pub reason: String,
}

impl FromKDL<'_> for Blocked {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_simple_node() {
        let input = r#"blocked "This is the reason""#;
        let actual = Blocked::from_kdl(input).unwrap();
        let expected = Blocked {
            reason: "This is the reason".into(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_empty_node() {
        let input = r"";
        let actual = Blocked::from_kdl(input).unwrap_err();
        let expected_message = "empty document";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_missing_until_property() {
        let input = r"blocked";
        let actual = Blocked::from_kdl(input).unwrap_err();
        let expected_message = "in Blocked: missing required argument at index 0";
        assert_eq!(actual.to_string(), expected_message);
    }
}
