use club_kdl::KdlDeserialize;

use crate::{
    bridge::FromKDL,
    properties::{deadline::Deadline, scheduled::Scheduled, sleeping::Sleeping, waiting::Waiting},
    types::datetime::TrayDateTime,
};

#[derive(KdlDeserialize, Debug, Eq, PartialEq)]
#[kdl(name = "done")]
pub struct Done {
    #[kdl(argument)]
    pub header: String,
    #[kdl(property)]
    pub completed: TrayDateTime,
    #[kdl(child(name = "sleeping"))]
    pub sleeping: Option<Sleeping>,
    #[kdl(child(name = "body"), unwrap_arg)]
    pub body: Option<String>,
    #[kdl(child(name = "scheduled"))]
    pub schedulded: Option<Scheduled>,
    #[kdl(child(name = "deadline"))]
    pub deadline: Option<Deadline>,
    #[kdl(child(name = "waiting"))]
    pub waiting: Option<Waiting>,
}

impl FromKDL<'_> for Done {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::datetime::TrayDateTime;

    use super::*;

    #[test]
    fn test_read_basic_done() {
        let input = r#"done "task header" completed="2026-10-15""#;
        let actual = Done::from_kdl(input).unwrap();
        let expected = Done {
            header: "task header".into(),
            sleeping: None,
            body: None,
            schedulded: None,
            deadline: None,
            waiting: None,
            completed: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_sleeping_done() {
        let input = r#"done "task header" completed="2026-10-15" {
            sleeping until="2026-10-15"
        }"#;
        let actual = Done::from_kdl(input).unwrap();
        let expected = Done {
            header: "task header".into(),
            sleeping: Some(Sleeping {
                until: TrayDateTime::from_str("2026-10-15").unwrap(),
            }),
            body: None,
            schedulded: None,
            deadline: None,
            waiting: None,
            completed: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_scheduled_and_deadlined_done() {
        let input = r#"done "task header" completed="2026-10-15" {
            scheduled "2026-10-15"
            deadline "2026-10-16"
        }"#;
        let actual = Done::from_kdl(input).unwrap();
        let expected = Done {
            header: "task header".into(),
            sleeping: None,
            body: None,
            schedulded: Some(Scheduled {
                on: TrayDateTime::from_str("2026-10-15").unwrap(),
            }),
            deadline: Some(Deadline {
                on: TrayDateTime::from_str("2026-10-16").unwrap(),
            }),
            waiting: None,
            completed: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_waiting_done_with_body() {
        let input = r#"done "task header" completed="2026-10-15" {
            body "task body"
            waiting since="2026-10-05"
        }"#;
        let actual = Done::from_kdl(input).unwrap();
        let expected = Done {
            header: "task header".into(),
            sleeping: None,
            body: Some("task body".into()),
            schedulded: None,
            deadline: None,
            waiting: Some(Waiting {
                since: TrayDateTime::from_str("2026-10-05").unwrap(),
            }),
            completed: TrayDateTime::from_str("2026-10-15").unwrap(),
        };
        assert_eq!(actual, expected);
    }
}
