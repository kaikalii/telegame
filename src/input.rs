use std::collections::BTreeSet;

use glam::Vec2;
use serde::Deserialize;

use crate::utils::ser_vec2;

#[derive(Debug, Default, Deserialize)]
pub struct Input {
    pub closed: bool,
    #[serde(with = "ser_vec2")]
    pub mouse_pos: Vec2,
    #[serde(with = "ser_vec2")]
    pub window_size: Vec2,
    pub key_events: Vec<KeyEvent>,
    pub keys_down: BTreeSet<String>,
    pub dt: f32,
}

impl Input {
    pub fn key_pressed(&self, key: &str) -> bool {
        self.key_events
            .iter()
            .any(|e| e.pressed && e.key == key && !e.repeat)
    }
    pub fn key_released(&self, key: &str) -> bool {
        self.key_events.iter().any(|e| !e.pressed && e.key == key)
    }
    pub fn key_down(&self, key: &str) -> bool {
        self.keys_down.contains(key)
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyEvent {
    pub key: String,
    pub pressed: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
    pub repeat: bool,
}
