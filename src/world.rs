use bevy::prelude::*;

use crate::world::{geometry::setup_geometry, level::setup_omgo_level};

mod geometry;
mod level;

#[allow(unused)]
#[derive(Default)]
pub enum WorldType {
    #[default]
    CustomGeometry,
    OmgoLevel,
}

pub struct WorldPlugin {
    world_type: WorldType,
}

impl WorldPlugin {
    pub fn new(world_type: WorldType) -> Self {
        Self { world_type }
    }
}

impl Default for WorldPlugin {
    fn default() -> Self {
        Self {
            world_type: WorldType::default(),
        }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        match self.world_type {
            WorldType::CustomGeometry => app.add_systems(Startup, setup_geometry),
            WorldType::OmgoLevel => app.add_systems(Startup, setup_omgo_level),
        };
    }
}
