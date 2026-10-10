use club_kdl::KdlDeserialize;

use crate::{bridge::FromKDL, properties::sleeping::Sleeping};

#[derive(KdlDeserialize, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::datetime::TrayDateTime;

    use super::*;

    #[test]
    fn test_read_basic_capture() {
        let input = r#"capture "capture header""#;
        let actual = club_kdl::from_str::<Capture>(input).unwrap();
        let expected = Capture {
            header: "capture header".into(),
            sleeping: None,
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_sleeping_capture() {
        let input = r#"capture "capture header" {
            sleeping until="2026-10-14"
        }"#;
        let actual = club_kdl::from_str::<Capture>(input).unwrap();
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
    fn test_read_body_capture() {
        let input = r#"capture "capture header" {
            body "more ideas on the capture header"
        }"#;
        let actual = club_kdl::from_str::<Capture>(input).unwrap();
        let expected = Capture {
            header: "capture header".into(),
            sleeping: None,
            body: Some("more ideas on the capture header".into()),
        };
        assert_eq!(actual, expected);
    }
}
