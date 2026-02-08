// NOTE: this is just a temporary setup to create world geometry before I incorporate a level
// editor

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    objects::entities::{Door, DoorShader},
    physics::ObjectLayer,
};

const WALL_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct Wall;

pub fn setup_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut door_materials: ResMut<Assets<DoorShader>>,
) {
    // Left walls
    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 275.0),
        vec2(-300.0, 250.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 275.0),
        vec2(-300.0, 0.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 275.0),
        vec2(-300.0, -250.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    // Right walls
    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 275.0),
        vec2(300.0, 250.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 75.0),
        vec2(300.0, 100.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    spawn_door(
        vec2(17.5, 125.0),
        vec2(300.0, 0.0),
        vec2(0.0, 120.0),
        0.0,
        &mut commands,
        &mut meshes,
        &mut door_materials,
    );

    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 75.0),
        vec2(300.0, -100.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(25.0, 275.0),
        vec2(300.0, -250.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    // Horizontal walls
    commands.spawn(rectangle_wall_bundle(
        vec2(625.0, 25.0),
        vec2(0.0, -375.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(625.0, 25.0),
        vec2(0.0, 375.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(250.0, 25.0),
        vec2(-187.5, 125.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(250.0, 25.0),
        vec2(187.5, 125.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    spawn_door(
        vec2(125.0, 17.5),
        vec2(0.0, 125.0),
        vec2(-120.0, 0.0),
        0.0,
        &mut commands,
        &mut meshes,
        &mut door_materials,
    );

    commands.spawn(rectangle_wall_bundle(
        vec2(250.0, 25.0),
        vec2(-187.5, -125.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    commands.spawn(rectangle_wall_bundle(
        vec2(250.0, 25.0),
        vec2(187.5, -125.0),
        0.0,
        &mut meshes,
        &mut color_materials,
    ));

    spawn_door(
        vec2(125.0, 17.5),
        vec2(0.0, -125.0),
        vec2(120.0, 0.0),
        0.0,
        &mut commands,
        &mut meshes,
        &mut door_materials,
    );
}

fn rectangle_wall_bundle(
    size: Vec2,
    position: Vec2,
    // Degrees
    angle: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
        Transform::from_xyz(position.x, position.y, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(angle))),
        Collider::rectangle(size.x, size.y),
        CollisionLayers::new(
            LayerMask(ObjectLayer::Obstacle.to_bits()),
            LayerMask(ObjectLayer::None.to_bits()),
        ),
        Wall,
    )
}

fn spawn_door(
    size: Vec2,
    position: Vec2,
    open_offset: Vec2,
    // Degrees
    angle: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<DoorShader>>,
) {
    // TODO: look for a better way to get the entity id
    let door_entity_id = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            Transform::from_xyz(position.x, position.y, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(angle))),
            Collider::rectangle(size.x, size.y),
            CollisionLayers::new(
                LayerMask(ObjectLayer::Door.to_bits()),
                LayerMask(ObjectLayer::None.to_bits()),
            ),
            Door::default().with_open_offset(open_offset),
        ))
        .id();
    commands.entity(door_entity_id).insert(MeshMaterial2d(
        materials.add(DoorShader::new(door_entity_id)),
    ));
}
