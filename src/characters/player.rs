use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    characters::Velocity,
    debug::CameraZoom,
    physics::{ObjectLayer, add_collision_layers},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerMovement>()
            .add_systems(Startup, player_setup)
            .add_systems(Update, player_input)
            .add_systems(FixedPreUpdate, player_movement)
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

#[derive(Message)]
enum PlayerMovement {
    Move(Vec2),
    Rotate(f32),
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use ObjectLayer as OL;

    commands.spawn((Camera2d, CameraZoom, PlayerCamera));
    commands.spawn((
        // NOTE: maybe one day capsules/other shapes with flat surfaces will work well, but not
        // today :(((
        // Mesh2d(meshes.add(Capsule2d::new(35.0, 70.0))),
        // Collider::capsule(35.0, 70.0),
        Mesh2d(meshes.add(Circle::new(30.0))),
        Collider::circle(30.0),
        add_collision_layers(vec![OL::Player], vec![OL::Obstacle]),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Velocity::default(),
        Player::default(),
    ));
}

fn player_input(
    key_input: Res<ButtonInput<KeyCode>>,
    player: Single<&Player>,
    mut player_movement_event: MessageWriter<PlayerMovement>,
) {
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

    if key_input.pressed(KeyCode::ArrowLeft) {
        player_movement_event.write(PlayerMovement::Rotate(2.0_f32.to_radians()));
    }
    if key_input.pressed(KeyCode::ArrowRight) {
        player_movement_event.write(PlayerMovement::Rotate(-2.0_f32.to_radians()));
    }

    if key_input.pressed(KeyCode::ShiftLeft) {
        velocity *= player.run_multiplier;
    }

    player_movement_event.write(PlayerMovement::Move(velocity));
}

fn player_movement(
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
    mut player_inputs: MessageReader<PlayerMovement>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    for event in player_inputs.read() {
        match event {
            PlayerMovement::Move(delta_velocity) => velocity.0 = *delta_velocity,
            PlayerMovement::Rotate(angle) => transform.rotate_z(*angle),
        }
    }
}

fn camera_tracking(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    camera.translation = player.translation;
}
