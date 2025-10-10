use avian2d::{parry::transformation::utils::transform, prelude::*};
use bevy::prelude::*;

use crate::{characters::Player, physics::ObjectLayer};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorOpen;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_doors)
            .add_systems(FixedUpdate, highlight_doors);
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
        Transform::from_xyz(0.0, 300.0, 0.0),
        Collider::rectangle(15.0, 100.0),
        CollisionLayers::new(
            LayerMask(OL::Obstacle.to_bits()),
            LayerMask(OL::None.to_bits()),
        ),
        Door,
    ));
}

fn highlight_doors(
    player: Single<&Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut doors: Query<(&mut Transform, Entity, Has<DoorOpen>), (With<Door>, Without<Player>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut door_transform, door_entity, is_open) in &mut doors {
        let mut entity = commands.entity(door_entity);

        // HACK: this sucks fuck, use vertex coloring or something
        if player
            .translation
            .xy()
            .distance(door_transform.translation.xy())
            <= 100.0
        {
            entity.insert(MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))));

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
        } else {
            entity.insert(MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))));
        }
    }
}
