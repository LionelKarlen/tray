use club_kdl::{KdlDeserialize, KdlSerialize};

use crate::{
    bridge::{FromKDL, ToKDL},
    properties::{
        blocked::Blocked, deadline::Deadline, scheduled::Scheduled, sleeping::Sleeping,
        waiting::Waiting,
    },
};

#[derive(KdlDeserialize, Debug, Eq, PartialEq, KdlSerialize)]
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
impl ToKDL for Next {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::datetime::TrayDateTime;

    use super::*;

    #[test]
    fn read_basic_task() {
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
    fn read_sleeping_task() {
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
    fn read_scheduled_and_deadlined_task() {
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
    fn read_waiting_task_with_body() {
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
    fn read_blocked_task() {
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

    fn round_trip(input: &str) {
        let intermediary = Next::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }

    #[test]
    fn write_basic_task() {
        let input = r#"next "task header""#;
        round_trip(input);
    }

    #[test]
    fn write_sleeping_task() {
        let input = r#"
next "task header" {
    sleeping until="2026-10-15"
}"#
        .trim_start();
        round_trip(input);
    }

    #[test]
    fn write_scheduled_and_deadlined_task() {
        let input = r#"
next "task header" {
    scheduled "2026-10-15"
    deadline "2026-10-16"
}"#
        .trim_start();
        round_trip(input);
    }

    #[test]
    fn write_waiting_task_with_body() {
        let input = r#"
next "task header" {
    body "task body"
    waiting since="2026-10-05"
}"#
        .trim_start();
        round_trip(input);
    }

    #[test]
    fn write_blocked_task() {
        let input = r#"
next "task header" {
    blocked "good reason"
}"#
        .trim_start();
        round_trip(input);
    }
}
