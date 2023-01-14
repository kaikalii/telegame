use glam::Vec2;
use serde::Serialize;

use crate::utils::ser_vec2;

#[derive(Default, Serialize)]
pub struct Frame {
    pub commands: Vec<Command>,
}

#[derive(Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Clear,
    Color {
        color: String,
    },
    Rectangle {
        #[serde(with = "ser_vec2")]
        pos: Vec2,
        #[serde(with = "ser_vec2")]
        size: Vec2,
    },
    Circle {
        #[serde(with = "ser_vec2")]
        pos: Vec2,
        radius: f32,
    },
}

impl Frame {
    pub fn clear(&mut self) {
        self.commands.push(Command::Clear);
    }
    pub fn color(&mut self, color: &str) {
        self.commands.push(Command::Color {
            color: color.into(),
        });
    }
    pub fn rectangle(&mut self, pos: Vec2, size: Vec2) {
        self.commands.push(Command::Rectangle { pos, size });
    }
    pub fn circle(&mut self, pos: Vec2, radius: f32) {
        self.commands.push(Command::Circle { pos, radius });
    }
}
