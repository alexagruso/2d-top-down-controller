mod player_shader;

pub use player_shader::*;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    debug::CameraZoom,
    objects::characters::{CharacterController, ControllerMovement},
    physics::{ObjectLayer, add_collision_layers},
    view_cone::ViewCone,
};

const PLAYER_TEXTURE_PATH: &str = "textures/placeholders/waltuh.tsx.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // TODO: move this message to the controller plugin
        app.add_message::<ControllerMovement>()
            .add_plugins(PlayerShaderPlugin)
            .add_systems(Startup, player_setup)
            .add_systems(Update, player_input)
            .add_systems(
                // PostUpdate prevents the camera from visually lagging behind the player
                FixedPostUpdate,
                camera_tracking,
            );
    }
}

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
pub struct Player {
    speed: f32,
    run_multiplier: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 100.0,
            run_multiplier: 2.5,
        }
    }
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlayerShader>>,
    assets: Res<AssetServer>,
) {
    use ObjectLayer as OL;

    commands.spawn((Camera2d, CameraZoom, PlayerCamera));
    commands.spawn((
        // Mesh2d(meshes.add(Capsule2d::new(35.0, 70.0))),
        // Collider::capsule(35.0, 70.0),
        Mesh2d(meshes.add(Circle::new(30.0))),
        Collider::circle(30.0),
        add_collision_layers(vec![OL::Player], vec![OL::Obstacle]),
        MeshMaterial2d(materials.add(PlayerShader {
            color: LinearRgba::WHITE,
            texture: Some(assets.load(PLAYER_TEXTURE_PATH)),
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Player::default(),
        ViewCone::new(450.0, f32::to_radians(75.0)).with_minimum_ray_spacing(f32::to_radians(0.1)),
        CharacterController,
    ));
}

fn player_input(
    key_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&Player, Entity)>,
    mut player_movement_event: MessageWriter<ControllerMovement>,
) {
    let (player, entity) = player.into_inner();
    let mut velocity = Vec2::ZERO;

    if key_input.pressed(KeyCode::KeyA) {
        velocity.x -= player.speed;
    }
    if key_input.pressed(KeyCode::KeyD) {
        velocity.x += player.speed;
    }

    if key_input.pressed(KeyCode::KeyW) {
        velocity.y += player.speed;
    }
    if key_input.pressed(KeyCode::KeyS) {
        velocity.y -= player.speed;
    }

    if key_input.pressed(KeyCode::KeyQ) {
        player_movement_event.write(ControllerMovement::from_rotation(
            2.0_f32.to_radians(),
            entity,
        ));
    }
    if key_input.pressed(KeyCode::KeyE) {
        player_movement_event.write(ControllerMovement::from_rotation(
            -2.0_f32.to_radians(),
            entity,
        ));
    }

    if key_input.pressed(KeyCode::ShiftLeft) {
        velocity *= player.run_multiplier;
    }

    player_movement_event.write(ControllerMovement::from_translation(velocity, entity));
}

// TODO: move this to a better place
fn camera_tracking(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    camera.translation = player.translation;
}
