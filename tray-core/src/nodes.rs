use club_kdl::{KdlDeserialize, KdlSerialize};

use crate::nodes::{
    capture::Capture, done::Done, event::Event, next::Next, project::Project, task::Task,
};

pub mod capture;
pub mod document;
pub mod done;
pub mod event;
pub mod next;
pub mod project;
pub mod task;

#[derive(PartialEq, Eq, Debug, KdlDeserialize, KdlSerialize)]
pub enum Node {
    Project(Project),
    Task(Task),
    Done(Done),
    Next(Next),
    Event(Event),
    Capture(Capture),
}
