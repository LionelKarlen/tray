use club_kdl::KdlDeserialize;

use crate::{bridge::FromKDL, nodes::Node};

#[derive(KdlDeserialize, Debug, Eq, PartialEq, Default)]
#[kdl(document)]
pub struct Document {
    #[kdl(children)]
    pub nodes: Vec<Node>,
}

impl FromKDL<'_> for Document {
    fn from_kdl(text: &str) -> Result<Self, club_kdl::Error>
    where
        for<'a> Self: KdlDeserialize<'a>,
    {
        if text.is_empty() {
            Ok(Document::default())
        } else {
            club_kdl::from_str(text)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::nodes::{done::Done, event::Event, task::Task};

    use super::*;

    #[test]
    fn test_read_empty_document() {
        let input = "";
        let actual = Document::from_kdl(input).unwrap();
        let expected = Document { nodes: vec![] };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_mixed_document() {
        let input = r#"task "I am a task"
        done "completed unrelated task" completed="2026-10-10"
        event "test event" on="2026-10-11"
        "#;
        let actual = Document::from_kdl(input).unwrap();
        let expected = Document {
            nodes: vec![
                Node::Task(Task {
                    header: "I am a task".into(),
                    blocked: None,
                    sleeping: None,
                    body: None,
                    schedulded: None,
                    deadline: None,
                    waiting: None,
                }),
                Node::Done(Done {
                    header: "completed unrelated task".into(),
                    completed: crate::types::datetime::TrayDateTime::from_str("2026-10-10")
                        .unwrap(),
                    sleeping: None,
                    body: None,
                    schedulded: None,
                    deadline: None,
                    waiting: None,
                }),
                Node::Event(Event {
                    header: "test event".into(),
                    on: crate::types::datetime::TrayDateTime::from_str("2026-10-11").unwrap(),
                    duration: None,
                    location: None,
                    body: None,
                }),
            ],
        };
        assert_eq!(actual, expected);
    }
}
