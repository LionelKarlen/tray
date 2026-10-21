use club_kdl::{KdlDeserialize, KdlSerialize};

use crate::{
    bridge::{FromKDL, ToKDL},
    properties::sleeping::Sleeping,
};

#[derive(KdlDeserialize, Debug, Eq, PartialEq, KdlSerialize)]
#[kdl(name = "capture")]
pub struct Capture {
    #[kdl(argument)]
    pub header: String,
    #[kdl(child(name = "sleeping"))]
    pub sleeping: Option<Sleeping>,
    #[kdl(child(name = "body"), unwrap_arg)]
    pub body: Option<String>,
}

impl FromKDL<'_> for Capture {}
impl ToKDL for Capture {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::datetime::TrayDateTime;

    use super::*;

    #[test]
    fn read_basic_capture() {
        let input = r#"capture "capture header""#;
        let actual = Capture::from_kdl(input).unwrap();
        let expected = Capture {
            header: "capture header".into(),
            sleeping: None,
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_sleeping_capture() {
        let input = r#"capture "capture header" {
            sleeping until="2026-10-14"
        }"#;
        let actual = Capture::from_kdl(input).unwrap();
        let expected = Capture {
            header: "capture header".into(),
            sleeping: Some(Sleeping {
                until: TrayDateTime::from_str("2026-10-14").unwrap(),
            }),
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_body_capture() {
        let input = r#"capture "capture header" {
            body "more ideas on the capture header"
        }"#;
        let actual = Capture::from_kdl(input).unwrap();
        let expected = Capture {
            header: "capture header".into(),
            sleeping: None,
            body: Some("more ideas on the capture header".into()),
        };
        assert_eq!(actual, expected);
    }

    fn round_trip(input: &str) {
        let intermediary = Capture::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }

    #[test]
    fn write_simple_capture() {
        let input = r#"capture "capture header""#;
        round_trip(input);
    }

    #[test]
    fn write_sleeping_capture() {
        let input = r#"
capture "capture header" {
    sleeping until="2026-10-14"
}"#
        .trim_start();
        round_trip(input);
    }

    #[test]
    fn write_body_capture() {
        let input = r#"
capture "capture header" {
    body "more ideas on the capture header"
}"#
        .trim_start();
        round_trip(input);
    }
}
