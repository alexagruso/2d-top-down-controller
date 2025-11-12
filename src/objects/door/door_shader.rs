use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::{
    characters::Player,
    objects::door::{DOOR_DEFAULT_FILL_COLOR, Door, DoorIsNear},
};

const DOOR_SHADER_PATH: &str = "shaders/door.wgsl";

pub struct DoorShaderPlugin;

impl Plugin for DoorShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DoorShader>::default())
            .add_systems(Update, update_door_shaders);
    }
}

fn update_door_shaders(
    doors: Query<(&Door, Has<DoorIsNear>)>,
    player: Single<&Transform, With<Player>>,
    mut door_highlight_shaders: ResMut<Assets<DoorShader>>,
) {
    for (_, shader) in door_highlight_shaders.iter_mut() {
        let (door, is_highlighted) = doors
            .get(shader.door_entity)
            .expect("Door shaders are always spawned with a corresponding door object.");

        shader.fill_color = if is_highlighted {
            door.highlight_color
        } else {
            door.fill_color
        };

        shader.player_position = player.translation.xy();
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct DoorShader {
    #[uniform(0)]
    fill_color: LinearRgba,
    #[uniform(1)]
    player_position: Vec2,
    door_entity: Entity,
}

impl DoorShader {
    pub fn new(entity: Entity) -> Self {
        Self {
            // Blue, full opacity
            fill_color: DOOR_DEFAULT_FILL_COLOR,
            player_position: Vec2::ZERO,
            door_entity: entity,
        }
    }
}

impl Material2d for DoorShader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        DOOR_SHADER_PATH.into()
    }
}
