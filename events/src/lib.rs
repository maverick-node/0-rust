use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        write!(f, "({})", self.content.truecolor(r, g, b))
    }
}

use Event::*;

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) =>
                Notification {
                    size: 50,
                    color: (50, 50, 50),
                    position: Position::Bottom,
                    content: text.to_string(),
                },
            Event::Registration(duration) => {
                let time_sec = duration.num_seconds();
                let hours = (time_sec / 3600) as u32;
                let minutes = ((time_sec % 3600) / 60) as u32;
                let seconds = (time_sec % 60) as u32;
                let formatted_duration = format!("{}H:{}M:{}S", hours, minutes, seconds);

                Notification {
                    size: 30,

                    color: (255, 2, 22),

                    position: Position::Top,

                    content: format!("You have {} left before the registration ends", formatted_duration),
                }
            }
            Event::Appointment(text) =>
                Notification {
                    size: 100,

                    color: (200, 200, 3),

                    position: Position::Center,

                    content: text.to_string(),
                },
            Event::Holiday =>
                Notification {
                    size: 25,

                    color: (0, 255, 0),

                    position: Position::Top,

                    content: "Enjoy your holiday".to_string(),
                },
        }
    }
}
