use club_kdl::{KdlDeserialize, KdlSerialize};

use crate::{
    bridge::{FromKDL, ToKDL},
    types::{datetime::TrayDateTime, duration::Duration},
};

#[derive(KdlDeserialize, Debug, Eq, PartialEq, KdlSerialize)]
#[kdl(name = "event")]
pub struct Event {
    #[kdl(argument)]
    pub header: String,
    #[kdl(property)]
    pub on: TrayDateTime,
    #[kdl(child(name = "duration"), unwrap_arg)]
    pub duration: Option<Duration>,
    #[kdl(child(name = "location"), unwrap_arg)]
    pub location: Option<String>,
    #[kdl(child(name = "body"), unwrap_arg)]
    pub body: Option<String>,
}

impl FromKDL<'_> for Event {}
impl ToKDL for Event {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn read_basic_event() {
        let input = r#"event "event description" on="2026-10-15""#;
        let actual = Event::from_kdl(input).unwrap();
        let expected = Event {
            header: "event description".into(),
            on: TrayDateTime::from_str("2026-10-15").unwrap(),
            duration: None,
            location: None,
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_duration_event() {
        let input = r#"event "event description" on="2026-10-15" {
            duration "3h"
        }"#;
        let actual = Event::from_kdl(input).unwrap();
        let expected = Event {
            header: "event description".into(),
            on: TrayDateTime::from_str("2026-10-15").unwrap(),
            duration: Some(Duration::from_str("3h").unwrap()),
            location: None,
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_location_event() {
        let input = r#"event "event description" on="2026-10-15" {
            location "somewhere"
        }"#;
        let actual = Event::from_kdl(input).unwrap();
        let expected = Event {
            header: "event description".into(),
            on: TrayDateTime::from_str("2026-10-15").unwrap(),
            duration: None,
            location: Some("somewhere".into()),
            body: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn read_malformed_event() {
        let input = r#"event "event description""#;
        let actual = Event::from_kdl(input).unwrap_err();
        let expected_message = "in Event: missing required field: on";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn read_malformed_duration() {
        let input = r#"event "event description" on="2026-10-10" {
            duration
        }"#;
        let actual = Event::from_kdl(input).unwrap();
        let expected = Event {
            header: "event description".into(),
            on: TrayDateTime::from_str("2026-10-10").unwrap(),
            duration: None,
            location: None,
            body: None,
        };
        assert_eq!(actual, expected);
    }

    fn round_trip(input: &str) {
        let intermediary = Event::from_kdl(input).unwrap();
        let actual = intermediary.to_kdl().unwrap();
        assert_eq!(actual, input);
    }

    #[test]
    fn write_basic_event() {
        let input = r#"event "event description" on="2026-10-15""#;
        round_trip(input);
    }

    #[test]
    fn write_duration_event() {
        let input = r#"
event "event description" on="2026-10-15" {
    duration "3h"
}"#
        .trim_start();
        round_trip(input);
    }

    #[test]
    fn write_location_event() {
        let input = r#"
event "event description" on="2026-10-15" {
    location somewhere
}"#
        .trim_start();
        round_trip(input);
    }
}
