use club_kdl::KdlDeserialize;

use crate::{
    bridge::FromKDL,
    nodes::{done::Done, next::Next, task::Task},
};

#[derive(PartialEq, Eq, Debug, KdlDeserialize)]
pub enum ProjectChild {
    Project(Project),
    Task(Task),
    Done(Done),
    Next(Next),
}

#[derive(KdlDeserialize, Debug, Eq, PartialEq)]
#[kdl(name = "project")]
pub struct Project {
    #[kdl(argument)]
    pub header: String,
    #[kdl(children)]
    pub children: Vec<ProjectChild>,
}

impl FromKDL<'_> for Project {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_empty_project() {
        let input = r#"project "project name""#;
        let actual = Project::from_kdl(input).unwrap();
        let expected = Project {
            header: "project name".into(),
            children: vec![],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_mixed_project() {
        let input = r#"project "project name" {
            done "done task" completed="2026-10-10"
            next "next task"
            task "following task"
        }"#;
        let actual = Project::from_kdl(input).unwrap();
        let expected = Project {
            header: "project name".into(),
            children: vec![
                ProjectChild::Done(
                    Done::from_kdl(r#"done "done task" completed="2026-10-10""#).unwrap(),
                ),
                ProjectChild::Next(Next::from_kdl(r#"next "next task""#).unwrap()),
                ProjectChild::Task(Task::from_kdl(r#"task "following task""#).unwrap()),
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_nested_project() {
        let input = r#"project "project name" {
            done "done task" completed="2026-10-10"
            next "next task"
            task "following task"
            project "important subproject" {
                done "wander aimlessly" completed="2026-10-11"
                next "look at a map"
                task "meander aimlessly anyway"
            }
        }"#;
        let actual = Project::from_kdl(input).unwrap();
        let expected = Project {
            header: "project name".into(),
            children: vec![
                ProjectChild::Done(
                    Done::from_kdl(r#"done "done task" completed="2026-10-10""#).unwrap(),
                ),
                ProjectChild::Next(Next::from_kdl(r#"next "next task""#).unwrap()),
                ProjectChild::Task(Task::from_kdl(r#"task "following task""#).unwrap()),
                ProjectChild::Project(Project {
                    header: "important subproject".into(),
                    children: vec![
                        ProjectChild::Done(
                            Done::from_kdl(r#"done "wander aimlessly" completed="2026-10-11""#)
                                .unwrap(),
                        ),
                        ProjectChild::Next(Next::from_kdl(r#"next "look at a map""#).unwrap()),
                        ProjectChild::Task(
                            Task::from_kdl(r#"task "meander aimlessly anyway""#).unwrap(),
                        ),
                    ],
                }),
            ],
        };
        assert_eq!(actual, expected);
    }
}
