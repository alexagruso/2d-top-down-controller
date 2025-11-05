use avian2d::prelude::*;
use bevy::prelude::*;

use crate::physics::ObjectLayer;

pub fn setup_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use ObjectLayer as OL;

    // commands.spawn((
    //     Mesh2d(meshes.add(ConvexPolygon)),
    // ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 150.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(-250.0, 0.0, 0.0),
        Collider::rectangle(25.0, 150.0),
        CollisionLayers::new(
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
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
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
        ),
        Wall,
    ));

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(25.0, 150.0))),
    //     MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    //     Transform::from_xyz(0.0, 250.0, 0.0)
    //         .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
    //     Collider::rectangle(25.0, 150.0),
    //     CollisionLayers::new(
    //         LayerMask(OL::Obstacle.to_bits()),
    //         LayerMask(OL::None.to_bits()),
    //     ),
    //     Wall,
    // ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(450.0, 25.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Collider::rectangle(450.0, 25.0),
        CollisionLayers::new(
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
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
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
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
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
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
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
            ),
            Wall,
        ));
    }
}

#[derive(Component)]
struct Wall;
