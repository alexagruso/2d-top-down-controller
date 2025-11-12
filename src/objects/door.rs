mod door_shader;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    characters::CharacterController,
    objects::door::door_shader::{DoorShader, DoorShaderPlugin},
    physics::ObjectLayer,
};

const DOOR_DEFAULT_FILL_COLOR: LinearRgba = LinearRgba::new(0.0, 0.0, 1.0, 1.0);
const DOOR_DEFAULT_HIGHLIGHT_COLOR: LinearRgba = LinearRgba::new(0.0, 0.25, 1.0, 1.0);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DoorOpen;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DoorIsNear;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DoorShaderPlugin)
            .add_systems(Startup, setup_doors)
            .add_systems(FixedUpdate, update_doors);
    }
}

#[derive(Component)]
struct Door {
    fill_color: LinearRgba,
    highlight_color: LinearRgba,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            // Blue, full opacity
            fill_color: DOOR_DEFAULT_FILL_COLOR,
            // Yellow, full opacity
            highlight_color: DOOR_DEFAULT_HIGHLIGHT_COLOR,
        }
    }
}

fn setup_doors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DoorShader>>,
) {
    use ObjectLayer as OL;

    // TODO: look for a better way to get the entity id
    let door_entity_id = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
            Transform::from_xyz(-20.0, 250.0, 0.0),
            Collider::rectangle(15.0, 100.0),
            CollisionLayers::new(
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
            ),
            Door::default(),
        ))
        .id();
    commands.entity(door_entity_id).insert(MeshMaterial2d(
        materials.add(DoorShader::new(door_entity_id)),
    ));

    let door_entity_id = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
            Transform::from_xyz(120.0, 250.0, 0.0),
            Collider::rectangle(15.0, 100.0),
            CollisionLayers::new(
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
            ),
            Door::default(),
        ))
        .id();
    commands.entity(door_entity_id).insert(MeshMaterial2d(
        materials.add(DoorShader::new(door_entity_id)),
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
) {
    for (mut door_transform, door_entity, is_open) in &mut doors {
        let mut entity = commands.entity(door_entity);

        let mut door_is_near = false;

        for controller_transform in &controllers {
            // TODO: make this use a collider attached to the door object rather than a simple
            // distance check to determine if a player is near
            if controller_transform
                .translation
                .xy()
                .distance(door_transform.translation.xy())
                <= 100.0
            {
                door_is_near = true;

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

        if door_is_near {
            entity.insert(DoorIsNear);
        } else {
            entity.remove::<DoorIsNear>();
        }
    }
}
