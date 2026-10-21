use club_kdl::KdlDeserialize;

use crate::{
    bridge::FromKDL,
    properties::{
        blocked::Blocked, deadline::Deadline, scheduled::Scheduled, sleeping::Sleeping,
        waiting::Waiting,
    },
};

#[derive(KdlDeserialize, Debug, Eq, PartialEq)]
#[kdl(name = "next")]
pub struct Next {
    #[kdl(argument)]
    pub header: String,
    #[kdl(child(name = "blocked"))]
    pub blocked: Option<Blocked>,
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

impl FromKDL<'_> for Next {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::datetime::TrayDateTime;

    use super::*;

    #[test]
    fn test_read_basic_task() {
        let input = r#"next "task header""#;
        let actual = Next::from_kdl(input).unwrap();
        let expected = Next {
            header: "task header".into(),
            sleeping: None,
            body: None,
            schedulded: None,
            deadline: None,
            waiting: None,
            blocked: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_sleeping_task() {
        let input = r#"next "task header" {
            sleeping until="2026-10-15"
        }"#;
        let actual = Next::from_kdl(input).unwrap();
        let expected = Next {
            header: "task header".into(),
            sleeping: Some(Sleeping {
                until: TrayDateTime::from_str("2026-10-15").unwrap(),
            }),
            body: None,
            schedulded: None,
            deadline: None,
            waiting: None,
            blocked: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_scheduled_and_deadlined_task() {
        let input = r#"next "task header" {
            scheduled "2026-10-15"
            deadline "2026-10-16"
        }"#;
        let actual = Next::from_kdl(input).unwrap();
        let expected = Next {
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
            blocked: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_waiting_task_with_body() {
        let input = r#"next "task header" {
            body "task body"
            waiting since="2026-10-05"
        }"#;
        let actual = Next::from_kdl(input).unwrap();
        let expected = Next {
            header: "task header".into(),
            sleeping: None,
            body: Some("task body".into()),
            schedulded: None,
            deadline: None,
            waiting: Some(Waiting {
                since: TrayDateTime::from_str("2026-10-05").unwrap(),
            }),
            blocked: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_blocked_task() {
        let input = r#"next "task header" {
            blocked "good reason"
        }"#;
        let actual = Next::from_kdl(input).unwrap();
        let expected = Next {
            header: "task header".into(),
            sleeping: None,
            body: None,
            schedulded: None,
            deadline: None,
            waiting: None,
            blocked: Some(Blocked {
                reason: "good reason".into(),
            }),
        };
        assert_eq!(actual, expected);
    }
}
