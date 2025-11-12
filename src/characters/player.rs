mod player_shader;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    characters::{
        CharacterController, ControllerMovement,
        player::player_shader::{PlayerShader, PlayerShaderPlugin},
    },
    debug::CameraZoom,
    physics::{ObjectLayer, add_collision_layers},
};

const PLAYER_TEXTURE_PATH: &str = "textures/Face.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
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
            color: LinearRgba::RED,
            texture: Some(assets.load(PLAYER_TEXTURE_PATH)),
        })),
        Transform::from_xyz(-50.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Player::default(),
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
