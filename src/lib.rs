// TODO: make a prelude for this crate

use crate::{
    characters::{CharacterControllerPlugin, OtherCharacterPlugin, PlayerPlugin},
    debug::DebugPlugin,
    objects::DoorPlugin,
    view_cone::ViewConePlugin,
    world::{WorldPlugin, WorldType},
};
use bevy::prelude::*;

pub mod characters;
pub mod debug;
pub mod objects;
pub mod physics;
pub mod view_cone;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugPlugin,
            CharacterControllerPlugin,
            ViewConePlugin,
            OtherCharacterPlugin,
            PlayerPlugin,
            WorldPlugin::new(WorldType::CustomGeometry),
            DoorPlugin,
        ));
    }
}
