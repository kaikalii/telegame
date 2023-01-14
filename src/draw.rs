use glam::Vec2;
use serde::Serialize;

use crate::utils::ser_vec2;

#[derive(Serialize)]
pub struct Frame {
    pub clear: bool,
    pub shapes: Vec<ColoredShape>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            clear: true,
            shapes: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
pub struct ColoredShape {
    #[serde(flatten)]
    pub shape: Shape,
    pub color: String,
}

#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Shape {
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

impl Shape {
    pub fn color(self, color: &str) -> ColoredShape {
        ColoredShape {
            shape: self,
            color: color.into(),
        }
    }
}

impl Frame {
    pub fn rectangle(&mut self, pos: Vec2, size: Vec2, color: &str) {
        self.shapes
            .push(Shape::Rectangle { pos, size }.color(color));
    }
    pub fn circle(&mut self, pos: Vec2, radius: f32, color: &str) {
        self.shapes.push(Shape::Circle { pos, radius }.color(color));
    }
}
