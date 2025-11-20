use bevy::prelude::*;

struct FreePlaceTile {
    position: Vec2,
    size: Vec2,
    rotation: f32,
    name: Option<String>,
    class: Option<String>,
}
