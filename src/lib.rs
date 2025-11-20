// TODO: make a prelude for this crate

use crate::{
    debug::DebugPlugin,
    objects::ObjectPlugin,
    view_cone::ViewConePlugin,
    world::{WorldPlugin, WorldType},
};
use bevy::prelude::*;

pub mod debug;
pub mod physics;
pub mod view_cone;
pub mod world;

pub mod objects {
    use bevy::prelude::*;

    use crate::objects::{
        characters::{CharacterControllerPlugin, OtherCharacterPlugin, PlayerPlugin},
        entities::DoorPlugin,
    };

    pub mod characters;
    pub mod entities;

    pub struct ObjectPlugin;

    // TODO: separate these into character/entity/... plugins
    impl Plugin for ObjectPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                CharacterControllerPlugin,
                PlayerPlugin,
                // OtherCharacterPlugin,
                DoorPlugin,
            ));
        }
    }
}

// TODO: maybe move this stuff to a vector submodule (only if the math module grows significantly)
pub mod math {
    use bevy::prelude::*;

    /// Converts a window position (top-left as origin, right-down as positive axes) to
    /// a viewport position (center as origin, right-up as positive axes)
    ///
    /// * `position` - Window position to be converted
    /// * `viewport_size` - Size of the viewport
    #[inline]
    pub fn window_to_viewport_position(position: Vec2, viewport_size: Vec2) -> Vec2 {
        vec2(
            position.x - viewport_size.x / 2.0,
            -position.y + viewport_size.y / 2.0,
        )
    }

    /// Rotates a vector counter-clockwise by a given angle in radians.
    ///
    /// * `v` - Vector to be rotated
    /// * `radians` - Angle to rotate the vector by
    #[inline]
    pub fn rotate_vec2_radians(v: Vec2, radians: f32) -> Vec2 {
        vec2(
            radians.cos() * v.x - radians.sin() * v.y,
            radians.sin() * v.x + radians.cos() * v.y,
        )
    }

    /// Rotates a vector counter-clockwise by a given angle in degrees.
    ///
    /// * `v` - Vector to be rotated
    /// * `degrees` - Angle to rotate the vector by
    #[inline]
    pub fn rotate_vec2_degrees(v: Vec2, degrees: f32) -> Vec2 {
        let radians = degrees.to_radians();
        vec2(
            radians.cos() * v.x - radians.sin() * v.y,
            radians.sin() * v.x + radians.cos() * v.y,
        )
    }
}

// pub mod physics {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugPlugin,
            ObjectPlugin,
            ViewConePlugin,
            WorldPlugin::new(WorldType::CustomGeometry),
        ));
    }
}
