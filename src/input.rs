use glam::Vec2;
use serde::Deserialize;

use crate::utils::ser_vec2;

#[derive(Default, Deserialize)]
pub struct Input {
    pub closed: bool,
    #[serde(with = "ser_vec2")]
    pub mouse_pos: Vec2,
    #[serde(with = "ser_vec2")]
    pub window_size: Vec2,
}
