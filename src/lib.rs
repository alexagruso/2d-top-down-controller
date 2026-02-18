// TODO: make a prelude for this crate

use crate::{
    debug::DebugPlugin,
    objects::{ObjectPlugin, entities::DoorMessage},
    sector::SectorPlugin,
    world::{WorldPlugin, WorldType},
};
use bevy::prelude::*;

pub mod debug;
pub mod mouse_cache;
pub mod physics;
pub mod sector;
pub mod world;

pub mod objects {
    use bevy::prelude::*;

    use crate::objects::{
        characters::{CharacterControllerPlugin, PlayerPlugin},
        entities::DoorPlugin,
    };

    pub mod characters;
    pub mod entities;

    pub struct ObjectPlugin;

    // TODO: separate these into character/entity/... plugins
    impl Plugin for ObjectPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((CharacterControllerPlugin, PlayerPlugin, DoorPlugin));
        }
    }
}

// TODO: maybe move this stuff to a vector submodule (only if the math module grows significantly)
pub mod math {
    use bevy::prelude::*;

    /// Rotates a vector counter-clockwise about the origin by a given angle in radians.
    ///
    /// * `v` - Vector to be rotated
    /// * `angle` - Angle in radians to rotate the vector by
    #[inline]
    pub fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
        vec2(
            angle.cos() * v.x - angle.sin() * v.y,
            angle.sin() * v.x + angle.cos() * v.y,
        )
    }

    /// Rotates a vector counter-clockwise about the origin by a given angle in degrees.
    ///
    /// * `v` - Vector to be rotated
    /// * `angle` - Angle in degrees to rotate the vector by
    #[inline]
    pub fn rotate_vec2_degrees(v: Vec2, angle: f32) -> Vec2 {
        rotate_vec2(v, angle.to_radians())
    }

    /// Rotates a vector counter-clockwise about a given point by a given angle in radians.
    ///
    /// * `v` - Vector to be rotated
    /// * `center` - Point to rotate about
    /// * `angle` - Angle in radians to rotate the vector by
    #[inline]
    pub fn rotate_vec2_about(v: Vec2, center: Vec2, angle: f32) -> Vec2 {
        rotate_vec2(v - center, angle) + center
    }

    /// Rotates a vector counter-clockwise about a given point by a given angle in degrees.
    ///
    /// * `v` - Vector to be rotated
    /// * `center` - Point to rotate about
    /// * `angle` - Angle in degrees to rotate the vector by
    #[inline]
    pub fn rotate_vec2_about_degrees(v: Vec2, center: Vec2, angle: f32) -> Vec2 {
        rotate_vec2(v - center, angle.to_radians()) + center
    }
}

// pub mod physics {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugPlugin,
            ObjectPlugin,
            SectorPlugin::<DoorMessage>::default(),
            WorldPlugin::new(WorldType::CustomGeometry),
        ));
    }
}
