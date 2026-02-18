mod door_shader;

use derive::SectorMessage;
pub use door_shader::*;

use bevy::{animation::AnimationTargetId, prelude::*};

use crate::{objects::characters::CharacterController, sector::SectorMessage};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorIsOpen;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorIsFocused;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DoorShaderPlugin)
            .init_resource::<DoorColors>()
            .add_message::<DoorMessage>()
            .add_systems(Update, update_doors)
            .add_systems(FixedUpdate, read_door_focused_message);
    }
}

#[derive(Message, SectorMessage)]
pub struct DoorMessage {
    entity: Entity,
}

impl From<Entity> for DoorMessage {
    fn from(value: Entity) -> Self {
        Self { entity: value }
    }
}

fn read_door_focused_message(
    mut doors: Query<Entity, With<Door>>,
    mut door_messages: MessageReader<DoorMessage>,
    mut commands: Commands,
) {
    let active_doors: Vec<Entity> = door_messages.read().map(|message| message.entity).collect();
    for door_entity in &mut doors {
        if active_doors.contains(&door_entity) {
            commands.entity(door_entity).insert(DoorIsFocused);
        } else {
            commands.entity(door_entity).remove::<DoorIsFocused>();
        }
    }
}

#[derive(Resource)]
pub struct DoorColors {
    pub fill_color: LinearRgba,
    pub focus_color: LinearRgba,
}

impl Default for DoorColors {
    fn default() -> Self {
        Self {
            fill_color: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            focus_color: LinearRgba::new(0.0, 0.25, 1.0, 1.0),
        }
    }
}

#[derive(Component)]
pub struct Door {
    fill_color: LinearRgba,
    focus_color: LinearRgba,
    offset: Vec2,
}

impl Door {
    pub fn new(fill_color: LinearRgba, focus_color: LinearRgba, offset: Vec2) -> Self {
        Self {
            fill_color,
            focus_color,
            offset,
        }
    }
}

// // Holds information about the animation we programmatically create.
// pub struct AnimationInfo {
//     // The name of the animation target (in this case, the text).
//     target_name: Name,
//     // The ID of the animation target, derived from the name.
//     target_id: AnimationTargetId,
//     // The animation graph asset.
//     graph: Handle<AnimationGraph>,
//     // The index of the node within that graph.
//     node_index: AnimationNodeIndex,
// }

// impl AnimationInfo {
//     fn create() -> AnimationInfo {
//
//     }
// }
//
//
// fn spawn_door() -> impl Bundle {
//     let AnimationInfo
// }

fn update_doors(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut doors: Query<
        (
            &mut Transform,
            &Door,
            Entity,
            Has<DoorIsFocused>,
            Has<DoorIsOpen>,
        ),
        Without<CharacterController>,
    >,
) {
    for (mut door_transform, door, door_entity_id, is_focused, is_open) in &mut doors {
        if !is_focused {
            continue;
        }

        let mut entity = commands.entity(door_entity_id);
        if keyboard.just_pressed(KeyCode::Space) {
            if is_open {
                entity.remove::<DoorIsOpen>();
                door_transform.translation -= door.offset.extend(0.0);
            } else {
                entity.insert(DoorIsOpen);
                door_transform.translation += door.offset.extend(0.0);
            }
        }
    }
}
