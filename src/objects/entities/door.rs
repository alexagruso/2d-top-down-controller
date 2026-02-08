// TODO: rename the shitty functions in here

mod door_shader;

pub use door_shader::*;

use bevy::prelude::*;

use crate::{objects::characters::CharacterController, sector::SectorMessage};

pub const DOOR_DEFAULT_FILL_COLOR: LinearRgba = LinearRgba::new(0.0, 0.0, 1.0, 1.0);
pub const DOOR_DEFAULT_HIGHLIGHT_COLOR: LinearRgba = LinearRgba::new(0.0, 0.25, 1.0, 1.0);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorOpen;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorIsNear;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DoorShaderPlugin)
            .add_message::<DoorMessage>()
            .add_systems(FixedUpdate, (update_doors, update_door_status_from_message));
    }
}

// TODO: Actually implement this
#[derive(Message)]
pub struct DoorMessage {
    entity: Entity,
}

impl From<Entity> for DoorMessage {
    fn from(value: Entity) -> Self {
        Self { entity: value }
    }
}

// TODO: see the sector message derive macro todo
impl SectorMessage for DoorMessage {}

fn update_door_status_from_message(
    mut doors: Query<(Entity, &mut Door)>,
    mut door_messages: MessageReader<DoorMessage>,
) {
    let active_doors: Vec<Entity> = door_messages.read().map(|m| m.entity).collect();

    for (entity, mut door) in &mut doors {
        // TODO: make this a marker component instead
        door.is_near = active_doors.contains(&entity);
    }
}

#[derive(Component)]
pub struct Door {
    fill_color: LinearRgba,
    highlight_color: LinearRgba,
    open_offset: Vec2,
    is_near: bool,
}

impl Door {
    pub fn with_open_offset(self, open_offset: Vec2) -> Self {
        Self {
            open_offset,
            ..self
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        Self {
            // Blue, full opacity
            fill_color: DOOR_DEFAULT_FILL_COLOR,
            // Yellow, full opacity
            highlight_color: DOOR_DEFAULT_HIGHLIGHT_COLOR,
            open_offset: Vec2::ZERO,
            is_near: false,
        }
    }
}

// fn on_door_

fn update_doors(
    controllers: Query<&Transform, With<CharacterController>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut doors: Query<(&mut Transform, &Door, Entity, Has<DoorOpen>), Without<CharacterController>>,
) {
    for (mut door_transform, door, door_entity_id, is_open) in &mut doors {
        let mut entity = commands.entity(door_entity_id);

        let mut door_is_near = false;

        for controller_transform in &controllers {
            // TODO: make this use a collider attached to the door object rather than a simple
            // distance check to determine if a player is near
            if door.is_near {
                door_is_near = true;

                // TODO: move this to an Update system that sends a door open message
                if keyboard.just_pressed(KeyCode::Space) {
                    if is_open {
                        entity.remove::<DoorOpen>();
                        door_transform.translation -= door.open_offset.extend(0.0);
                    } else {
                        entity.insert(DoorOpen);
                        door_transform.translation += door.open_offset.extend(0.0);
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
