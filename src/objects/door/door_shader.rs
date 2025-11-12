use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::objects::door::{DOOR_DEFAULT_FILL_COLOR, Door, DoorIsNear};

const DOOR_SHADER_PATH: &str = "shaders/door.wgsl";

pub struct DoorShaderPlugin;

impl Plugin for DoorShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DoorShader>::default())
            .add_systems(Update, update_door_shaders);
    }
}

fn update_door_shaders(
    door_is_open: Query<(&Door, Has<DoorIsNear>)>,
    mut door_highlight_shaders: ResMut<Assets<DoorShader>>,
) {
    for (_, shader) in door_highlight_shaders.iter_mut() {
        match door_is_open.get(shader.door_entity) {
            Ok((door, highlight)) => {
                shader.fill_color = if highlight {
                    door.highlight_color
                } else {
                    door.fill_color
                }
            }
            Err(_) => {
                unreachable!("Door shaders are always spawned with a corresponding door object.")
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct DoorShader {
    #[uniform(0)]
    fill_color: LinearRgba,
    door_entity: Entity,
}

impl DoorShader {
    pub fn new(entity: Entity) -> Self {
        Self {
            // Blue, full opacity
            fill_color: DOOR_DEFAULT_FILL_COLOR,
            door_entity: entity,
        }
    }
}

impl Material2d for DoorShader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        DOOR_SHADER_PATH.into()
    }
}
