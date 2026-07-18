use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({:?}, {}, {})" ,self.position,self.size,self.content)
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(msg) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: msg.truecolor(50, 50, 50).to_string(),
            },

            Event::Appointment(msg) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: msg.truecolor(200, 200, 3).to_string(),
            },

            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".truecolor(0, 255, 0).to_string(),
            },

            Event::Registration(d) => {
                let s = d.as_secs();

                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!("You have {}H:{}M:{}S left before the registration ends",s / 3600,(s % 3600) / 60,s % 60).truecolor(255, 2, 22).to_string(),
                }
            }
        }
    }
}