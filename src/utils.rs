pub mod ser_vec2 {
    use glam::Vec2;
    use serde::{Deserialize, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct Vec2Rep {
        x: f32,
        y: f32,
    }

    pub fn serialize<S>(v: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Vec2Rep { x: v.x, y: v.y }.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let Vec2Rep { x, y } = Vec2Rep::deserialize(deserializer)?;
        Ok(Vec2::new(x, y))
    }
}
