use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};

const PLAYER_SHADER_PATH: &str = "shaders/player.wgsl";

pub struct PlayerShaderPlugin;

impl Plugin for PlayerShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<PlayerShader>::default());
    }
}

// TODO: encapsulate these in a builder function
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct PlayerShader {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for PlayerShader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        PLAYER_SHADER_PATH.into()
    }
}
