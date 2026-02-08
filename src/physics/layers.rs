use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum ObjectLayer {
    #[default]
    None,
    Obstacle,
    Player,
    Door,
}

/// Awd
pub fn object_collision_layers(
    member_of: Vec<ObjectLayer>,
    collides_with: Vec<ObjectLayer>,
) -> impl Bundle {
    let mut members: u32 = 0;
    let mut filters: u32 = 0;

    for layer in &member_of {
        members |= layer.to_bits();
    }
    for layer in &collides_with {
        filters |= layer.to_bits();
    }

    CollisionLayers::new(LayerMask(members), LayerMask(filters))
}
