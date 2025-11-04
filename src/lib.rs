// TODO: make a prelude for this crate

use crate::{
    characters::{CharacterControllerPlugin, OtherCharacterPlugin, PlayerPlugin},
    debug::DebugPlugin,
    laser::LaserPlugin,
    objects::DoorPlugin,
    world::{WorldPlugin, WorldType},
};
use bevy::prelude::*;

pub mod characters;
pub mod debug;
pub mod laser;
pub mod objects;
pub mod physics;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugPlugin,
            CharacterControllerPlugin,
            LaserPlugin,
            OtherCharacterPlugin,
            PlayerPlugin,
            WorldPlugin::new(WorldType::CustomGeometry),
            DoorPlugin,
        ));
    }
}
