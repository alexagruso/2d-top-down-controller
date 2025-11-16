use std::fs::File;

use bevy::prelude::*;

pub struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
    }
}

pub fn setup_level(assets: Res<AssetServer>, mut commands: Commands) {}

#[derive(Asset, TypePath)]
struct Level {
    position: Vec<Vec2>,
}

struct LevelHandle(Handle<Level>);
