// TODO: change naming convention to character rather than player so that we can semantically apply
// this code to non-player entities.
// TODO: look into changing manual velocity/position movement to avian2d
// linearvelocity/angularvelocity components

use avian2d::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::mouse::MouseWheel, log::LogPlugin,
    math::InvalidDirectionError, prelude::*, window::PrimaryWindow,
};
use topdown_controller_2d::debug::FpsOverlayPlugin;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins((
            // Prevents non-error bevy engine logs from printing to the console
            DefaultPlugins.build().disable::<LogPlugin>(),
            PhysicsPlugins::default().with_length_unit(20.0),
            FrameTimeDiagnosticsPlugin::default(),
            PhysicsDebugPlugin::default(),
            FpsOverlayPlugin::default(),
        ))
        .add_event::<PlayerMovement>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (exit_on_esc, player_input, projectile_input, camera_zoom),
        )
        .add_systems(
            FixedUpdate,
            (
                (player_movement, player_collision_response).chain(),
                projectile_collision_response,
                // This prevents the camera from visually lagging behind the player
                camera_tracking.after(player_collision_response),
            ),
        )
        .run();
}

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
enum ObjectLayer {
    #[default]
    None,
    Obstacle,
    Player,
    Projectile,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::default(),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            // Position is considered with (0,0) as the top-left corner of the window
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        StatusText,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(35.0, 70.0))),
        Collider::capsule(35.0, 70.0),
        // Mesh2d(meshes.add(Circle::new(30.0))),
        // Collider::circle(30.0),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Velocity::default(),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Player.to_bits()),
            LayerMask(ObjectLayer::Obstacle.to_bits()),
        ),
        Player::default(),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 150.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(-250.0, 0.0, 0.0),
        Collider::rectangle(25.0, 150.0),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Obstacle.to_bits()),
            LayerMask(ObjectLayer::None.to_bits()),
        ),
        Wall,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 150.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(250.0, 50.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
        Collider::rectangle(25.0, 150.0),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Obstacle.to_bits()),
            LayerMask(ObjectLayer::None.to_bits()),
        ),
        Wall,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 150.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 250.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
        Collider::rectangle(25.0, 150.0),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Obstacle.to_bits()),
            LayerMask(ObjectLayer::None.to_bits()),
        ),
        Wall,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(450.0, 25.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Collider::rectangle(450.0, 25.0),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Obstacle.to_bits()),
            LayerMask(ObjectLayer::None.to_bits()),
        ),
        Wall,
    ));

    for i in 1..=7 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(25.0, 125.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(-300.0 - 285.0 * i as f32, 60.0, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(-10.0 * i as f32))),
            Collider::rectangle(25.0, 125.0),
            CollisionLayers::new(
                LayerMask(ObjectLayer::Obstacle.to_bits()),
                LayerMask(ObjectLayer::None.to_bits()),
            ),
            Wall,
        ));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(25.0, 125.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(-300.0 - 285.0 * i as f32, -60.0, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(10.0 * i as f32))),
            Collider::rectangle(25.0, 125.0),
            CollisionLayers::new(
                LayerMask(ObjectLayer::Obstacle.to_bits()),
                LayerMask(ObjectLayer::None.to_bits()),
            ),
            Wall,
        ));
    }

    for i in 0..5 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(150.0, 25.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, -400.0 + 50.0 * i as f32, 0.0)
                .with_rotation(Quat::from_rotation_z((i as f32 * 30.0).to_radians())),
            Collider::rectangle(150.0, 25.0),
            CollisionLayers::new(
                LayerMask(ObjectLayer::Obstacle.to_bits()),
                LayerMask(ObjectLayer::None.to_bits()),
            ),
            Wall,
        ));
    }
}

fn exit_on_esc(key_input: Res<ButtonInput<KeyCode>>, mut exit_event: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::Escape) {
        exit_event.write(AppExit::Success);
    }
}

// TODO: extract this into a component/plugin/whatever
fn camera_zoom(
    mut scroll_input: EventReader<MouseWheel>,
    mut camera: Query<&mut Projection, With<Camera2d>>,
) {
    // TODO: error handle this
    let mut projection = camera.single_mut().unwrap();
    for event in scroll_input.read() {
        // TODO: modify this to handle different scroll units
        if let Projection::Orthographic(projection) = projection.as_mut() {
            projection.scale = (projection.scale - event.y * 0.3).clamp(0.1, 5.0);
        }
    }
}

fn camera_tracking(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    // TODO: error handle these
    let player_transform = player.single().unwrap();
    let mut camera_transform = camera.single_mut().unwrap();

    camera_transform.translation = player_transform.translation;
}

#[derive(Component)]
struct Projectile {
    speed: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self { speed: 150.0 }
    }
}

fn cursor_to_camera_position(window_position: Vec2, window_size: Vec2) -> Vec2 {
    Vec2::new(
        window_position.x - window_size.x / 2.0,
        -window_position.y + window_size.y / 2.0,
    )
}

fn projectile_input(
    window: Single<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_transform = player.single().unwrap();
    let window_size = window.size();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let projectile_position = cursor_to_camera_position(cursor_position, window_size);
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(15.0))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                player_transform.clone(),
                Velocity(projectile_position.normalize()),
                Collider::circle(15.0),
                CollisionLayers::new(
                    LayerMask(ObjectLayer::Projectile.to_bits()),
                    LayerMask(ObjectLayer::Obstacle.to_bits()),
                ),
                Projectile::default(),
            ));
        }
    }
}

fn projectile_collision_response(
    time: Res<Time<Fixed>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut projectiles: Query<(
        &mut Transform,
        &Velocity,
        &Collider,
        &Projectile,
        &CollisionLayers,
        Entity,
    )>,
    mut commands: Commands,
) {
    for (mut transform, velocity, collider, projectile, collision_layers, entity) in
        &mut projectiles
    {
        let delta_velocity = velocity.0 * time.delta_secs();
        let direction = match Dir2::new(velocity.0) {
            Ok(result) => result,
            Err(_) => {
                // If the projectile has zero or NaN velocity, we will just delete it.
                commands.entity(entity).despawn();
                continue;
            }
        };

        if let Some(_) = spatial_query.cast_shape(
            &collider,
            transform.translation.xy(),
            0.0,
            direction,
            &ShapeCastConfig {
                max_distance: delta_velocity.length(),
                ..default()
            },
            &SpatialQueryFilter::from_mask(collision_layers.filters),
        ) {
            commands.entity(entity).despawn();
        } else {
            transform.translation += (delta_velocity * projectile.speed).extend(0.0);
        }
    }
}

#[derive(Component)]
struct StatusText;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
    run_multiplier: f32,
    // TODO: move this to the collide and slide config
    minimum_move_distance: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 100.0,
            run_multiplier: 2.5,
            minimum_move_distance: 0.5,
        }
    }
}

#[derive(Event)]
enum PlayerMovement {
    Move(Vec2),
    Rotate(f32),
}

#[derive(Component)]
struct Wall;

fn player_input(
    key_input: Res<ButtonInput<KeyCode>>,
    player: Query<&Player>,
    mut player_movement_event: EventWriter<PlayerMovement>,
) {
    let player = match player.single() {
        Ok(result) => result,
        Err(_) => panic!("There must always be exactly one player"),
    };

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
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut player_input_event: EventReader<PlayerMovement>,
) {
    let (mut transform, mut velocity) = match player_query.single_mut() {
        Ok(result) => result,
        Err(_) => panic!("There must always be exactly one player"),
    };

    for event in player_input_event.read() {
        match event {
            PlayerMovement::Move(delta_velocity) => velocity.0 = *delta_velocity,
            PlayerMovement::Rotate(angle) => transform.rotate_z(*angle),
        }
    }
}

struct CollideAndSlideConfig {
    skin_width: f32,
    bounces: usize,
}

impl Default for CollideAndSlideConfig {
    fn default() -> Self {
        CollideAndSlideConfig {
            skin_width: 0.1,
            bounces: 2,
        }
    }
}

// FIX: occasional jittering when moving a flat surface against a corner which is being caused by
// the
// HACK: limiting the bounces to 2 prevents the double-surface jittering issue, but I'd like to
// remove this limitation to handle arbitrarily detailed geometry
fn player_collision_response(
    time: Res<Time<Fixed>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut player_query: Query<(
        &mut Transform,
        &Velocity,
        &Collider,
        &CollisionLayers,
        &Player,
    )>,
    mut status_text: Query<&mut Text, With<StatusText>>,
    // mut gizmos: Gizmos,
) {
    let (mut transform, velocity, collider, collision_layers, player) =
        player_query.single_mut().unwrap();
    let player_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
    let player_angle_unit = vec2(player_angle.cos(), player_angle.sin());

    let mut cast_origin = transform.translation.xy();
    let mut cast_velocity = velocity.0 * time.delta_secs();
    // NOTE: not really necessary for top-down controllers, but if I ever modify this to be a
    // 2d platformer controller, we will need to initialize this as the player velocity and then
    // modify it only if we collide with something.
    let mut delta_velocity = Vec2::ZERO;

    let config = CollideAndSlideConfig::default();

    // TODO: extract this into a separate function
    'bounces: for _ in 0..config.bounces {
        let direction = match Dir2::new(cast_velocity) {
            Ok(result) => result,
            // HACK: If the velocity is zero, we set some dummy direction to satisfy the function
            // call. Maybe we don't need to do this; the spatial query pipeline object might have a
            // better function for this (i don't think it does)
            Err(InvalidDirectionError::Zero) => Dir2::X,
            Err(_) => panic!("cast velocity is either infinite or NaN"),
        };

        if let Some(hit) = spatial_query.cast_shape(
            &collider,
            cast_origin,
            player_angle,
            direction,
            &ShapeCastConfig {
                max_distance: cast_velocity.length() + config.skin_width,
                ..default()
            },
            &SpatialQueryFilter::from_mask(collision_layers.filters),
        ) {
            // Maximum distance the collider can move in the direction of the cast without
            // hitting another entity
            let snap_to_surface =
                cast_velocity.normalize_or_zero() * (hit.distance - config.skin_width).max(0.0);

            // If the hit distance is 0 the shapes are colliding and we need to handle it
            // separately
            if hit.distance > 0.0 {
                // Move collider as far as we can in the direction of the cast and calculate
                // remainder velocity for the next step
                delta_velocity += snap_to_surface;
                cast_origin += snap_to_surface;
                cast_velocity = (cast_velocity - snap_to_surface).reject_from(hit.normal1);
            } else {
                // Push the player out by the penetration depth
                // TODO: this isn't a perfect implementation; minor jittering when a flat surface
                // moves along a corner and inconsistent movement when the collider is moving
                // along a flat surface while rotating.
                let world_hit = hit.point1;
                let character_hit =
                    hit.point2.rotate(player_angle_unit) + transform.translation.xy();
                delta_velocity += (world_hit - character_hit) * 0.99;
            }
        } else {
            // No collision was detected, so we move the remaining distance and break the loop.
            delta_velocity += cast_velocity;
            break 'bounces;
        }
    }

    // TODO: move this to a dedicated debug system
    let mut text = status_text.single_mut().unwrap();
    **text = format!(
        "{:+.7}, {:+.7} \n{:.7}",
        delta_velocity.x, delta_velocity.y, player.minimum_move_distance
    );

    if delta_velocity.length() >= player.minimum_move_distance {
        transform.translation += delta_velocity.extend(0.0);
    }
}
