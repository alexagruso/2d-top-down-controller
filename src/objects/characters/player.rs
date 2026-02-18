mod player_shader;

use std::{f32::consts::PI, time::Duration};

pub use player_shader::*;

use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    debug::CameraZoom,
    objects::{
        characters::{CharacterController, ControllerMovement},
        entities::DoorMessage,
    },
    physics::{ObjectLayer, object_collision_layers},
    sector::{Sector, SectorTrigger},
};

const PLAYER_TEXTURE_PATH: &str = "textures/placeholders/topdown.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // TODO: move this message to the controller plugin
        app.add_message::<ControllerMovement>()
            .add_plugins(PlayerShaderPlugin)
            .add_systems(Startup, player_setup)
            .add_systems(
                Update,
                (
                    player_input,
                    rotate_player,
                    animate_sprite::<PlayerLegs>.run_if(player_is_moving),
                ),
            )
            .add_systems(FixedPostUpdate, camera_tracking);
    }
}

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct PlayerLegs;

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

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct AnimationTweenerThingy {
    factor: f32,
    constant: f32,
}

impl AnimationTweenerThingy {
    fn new(v1: f32, v2: f32, t1: f32, t2: f32) -> Self {
        let constant: f32 = (v2 * t2 - v1 * t1) / (v2 - v1);
        let factor: f32 = v1 * (t1 - constant);

        Self { factor, constant }
    }

    fn apply(&self, speed: f32) -> f32 {
        self.factor / speed + self.constant
    }
}

// TODO: this shouldn't really need to be general, remove the Marker type
fn animate_sprite<Marker: Component>(
    time: Res<Time>,
    player_velocity: Single<&LinearVelocity, With<Player>>,
    mut animations: Query<(&mut Sprite, &mut AnimationTimer, &AnimationIndices), With<Marker>>,
) {
    for (mut sprite, mut timer, indices) in &mut animations {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
            }
        }

        let frame_time =
            AnimationTweenerThingy::new(100.0, 250.0, 0.05, 0.035).apply(player_velocity.length());
        timer.set_duration(Duration::from_secs_f32(frame_time));
    }
}

fn player_is_moving(player_velocity: Query<&LinearVelocity, With<Player>>) -> bool {
    player_velocity
        .single()
        .expect("Should always be a player")
        .xy()
        != Vec2::ZERO
}

fn player_setup(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = assets.load(PLAYER_TEXTURE_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 20, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_indices = AnimationIndices {
        first: 20,
        last: 20,
    };
    let leg_indices = AnimationIndices { first: 0, last: 19 };

    commands.spawn((Camera2d, CameraZoom, PlayerCamera));
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_indices.first,
            }),
            // TODO: find a better way of scaling sprites up
            custom_size: Some(Vec2::splat(75.0)),
            ..default()
        },
        Collider::capsule(10.0, 25.0),
        object_collision_layers(
            vec![ObjectLayer::Player],
            vec![ObjectLayer::Obstacle, ObjectLayer::Door],
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Player::default(),
        CharacterController,
        children![
            (
                SectorTrigger::<DoorMessage>::new(Sector::new(75.0, PI * 0.35, 0.0, 8.0))
                    .with_mask(LayerMask(ObjectLayer::Door.to_bits()))
                    .into_bundle(&mut meshes),
            ),
            (
                PlayerLegs,
                Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: leg_indices.first,
                    }),
                    custom_size: Some(Vec2::splat(75.0)),
                    ..default()
                },
                leg_indices,
                AnimationTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            )
        ],
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

    if key_input.pressed(KeyCode::ShiftLeft) {
        velocity *= player.run_multiplier;
    }

    player_movement_event.write(ControllerMovement::from_translation(velocity, entity));
}

fn rotate_player(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    player_velocity: Query<&LinearVelocity, With<Player>>,
    mut legs_sprite: Query<(&mut Sprite, &AnimationIndices), With<PlayerLegs>>,
    // The QueryData type must be the same for Without to work
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut legs_transform: Query<&mut Transform, (With<PlayerLegs>, Without<Player>)>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();
    let cursor_world_position = match window.cursor_position() {
        Some(viewport_position) => camera
            .viewport_to_world_2d(camera_transform, viewport_position)
            .expect("Viewport to world conversion should never fail"),
        None => return Ok(()),
    };

    let mut player_transform = player_transform.single_mut()?;
    let mut legs_transform = legs_transform.single_mut()?;

    let player_position = player_transform.translation.xy();
    let player_angle = Vec2::X.angle_to(cursor_world_position - player_position);
    player_transform.rotation = Quat::from_rotation_z(player_angle);

    let player_velocity = player_velocity.single()?.xy();
    if player_velocity != Vec2::ZERO {
        // De-rotate by player angle then rotate according to velocity
        legs_transform.rotation = Quat::from_rotation_z(player_velocity.to_angle() - player_angle);
    } else {
        let (mut sprite, indices) = legs_sprite.single_mut()?;
        legs_transform.rotation = Quat::from_rotation_z(0.0);
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.first;
        }
    }

    Ok(())
}

// TODO: move this to a better place
fn camera_tracking(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    camera.translation = player.translation;
}
