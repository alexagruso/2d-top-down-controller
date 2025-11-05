use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{characters::CharacterController, physics::ObjectLayer};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorOpen;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_doors)
            .add_systems(FixedUpdate, update_doors);
    }
}

#[derive(Component)]
struct Door;

fn setup_doors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use ObjectLayer as OL;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(-20.0, 250.0, 0.0),
        Collider::rectangle(15.0, 100.0),
        CollisionLayers::new(
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
        ),
        Door,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(120.0, 250.0, 0.0),
        Collider::rectangle(15.0, 100.0),
        CollisionLayers::new(
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
        ),
        Door,
    ));
}

fn update_doors(
    controllers: Query<&Transform, With<CharacterController>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut doors: Query<
        (&mut Transform, Entity, Has<DoorOpen>),
        (With<Door>, Without<CharacterController>),
    >,
    // HACK: this sucks fuck, use vertex coloring or something
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut door_transform, door_entity, is_open) in &mut doors {
        let mut entity = commands.entity(door_entity);
        let mut highlight = false;

        for controller_transform in &controllers {
            if controller_transform
                .translation
                .xy()
                .distance(door_transform.translation.xy())
                <= 100.0
            {
                highlight = true;

                // TODO: move this to an Update system that sends a door open message
                if keyboard.just_pressed(KeyCode::Space) {
                    if is_open {
                        entity.remove::<DoorOpen>();
                        door_transform.translation.y -= 100.0;
                    } else {
                        entity.insert(DoorOpen);
                        door_transform.translation.y += 100.0;
                    }
                }
            }
        }

        if highlight {
            entity.insert(MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))));
        } else {
            entity.insert(MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))));
        }
    }
}
