use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    characters::{CharacterController, ControllerMovement, Velocity},
    physics::{ObjectLayer, add_collision_layers},
};

pub struct OtherCharacterPlugin;

impl Plugin for OtherCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ControllerMovement>()
            .add_systems(Startup, other_character_setup)
            .add_systems(Update, other_character_input);
    }
}
#[derive(Component)]
pub struct OtherCharacter {
    speed: f32,
    run_multiplier: f32,
}

impl Default for OtherCharacter {
    fn default() -> Self {
        Self {
            speed: 100.0,
            run_multiplier: 2.5,
        }
    }
}

fn other_character_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use ObjectLayer as OL;

    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(35.0, 70.0))),
        Collider::capsule(35.0, 70.0),
        // Mesh2d(meshes.add(Circle::new(30.0))),
        // Collider::circle(30.0),
        add_collision_layers(vec![OL::Player], vec![OL::Obstacle]),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.5))),
        Transform::from_xyz(50.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
        Velocity::default(),
        OtherCharacter::default(),
        CharacterController,
    ));

    // commands.spawn((
    //     Mesh2d(meshes.add(Capsule2d::new(35.0, 70.0))),
    //     Collider::capsule(35.0, 70.0),
    //     // Mesh2d(meshes.add(Circle::new(30.0))),
    //     // Collider::circle(30.0),
    //     add_collision_layers(vec![OL::Player], vec![OL::Obstacle]),
    //     MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.5))),
    //     Transform::from_xyz(-50.0, 0.0, 0.0)
    //         .with_rotation(Quat::from_rotation_z(f32::to_radians(0.0))),
    //     Velocity::default(),
    //     OtherCharacter::default(),
    //     CharacterController,
    // ));
}

fn other_character_input(
    key_input: Res<ButtonInput<KeyCode>>,
    other_characters: Query<(&OtherCharacter, Entity)>,
    mut movement_event: MessageWriter<ControllerMovement>,
) {
    for (character, entity) in &other_characters {
        let mut velocity = Vec2::ZERO;

        if key_input.pressed(KeyCode::KeyJ) {
            velocity.x -= character.speed;
        }
        if key_input.pressed(KeyCode::KeyL) {
            velocity.x += character.speed;
        }

        if key_input.pressed(KeyCode::KeyI) {
            velocity.y += character.speed;
        }
        if key_input.pressed(KeyCode::KeyK) {
            velocity.y -= character.speed;
        }

        if key_input.pressed(KeyCode::KeyU) {
            movement_event.write(ControllerMovement::from_rotation(
                2.0_f32.to_radians(),
                entity,
            ));
        }
        if key_input.pressed(KeyCode::KeyO) {
            movement_event.write(ControllerMovement::from_rotation(
                -2.0_f32.to_radians(),
                entity,
            ));
        }

        if key_input.pressed(KeyCode::ShiftLeft) {
            velocity *= character.run_multiplier;
        }

        movement_event.write(ControllerMovement::from_translation(velocity, entity));
    }
}
