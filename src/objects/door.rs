// TODO: extract the door shader logic to a different file

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};

const DOOR_HIGHLIGHT_SHADER_PATH: &str = "shaders/door_highlight.wgsl";

use crate::{characters::CharacterController, physics::ObjectLayer};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorOpen;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoorIsNear;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DoorHighlightMaterial>::default())
            .add_systems(Startup, setup_doors)
            .add_systems(FixedUpdate, update_doors)
            .add_systems(Update, update_door_shaders);
    }
}

#[derive(Component)]
struct Door;

fn setup_doors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DoorHighlightMaterial>>,
) {
    use ObjectLayer as OL;

    // TODO: look for a better way to get the entity id
    let door = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
            Transform::from_xyz(-20.0, 250.0, 0.0),
            Collider::rectangle(15.0, 100.0),
            CollisionLayers::new(
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
            ),
            Door,
        ))
        .id();
    commands.entity(door).insert(MeshMaterial2d(
        materials.add(DoorHighlightMaterial::new(door)),
    ));

    let door = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(15.0, 100.0))),
            Transform::from_xyz(120.0, 250.0, 0.0),
            Collider::rectangle(15.0, 100.0),
            CollisionLayers::new(
                LayerMask(OL::Obstacle.to_bits()),
                LayerMask(OL::None.to_bits()),
            ),
            Door,
        ))
        .id();
    commands.entity(door).insert(MeshMaterial2d(
        materials.add(DoorHighlightMaterial::new(door)),
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
            // distance check
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

fn update_door_shaders(
    door_is_open: Query<(), With<DoorIsNear>>,
    mut door_highlight_shaders: ResMut<Assets<DoorHighlightMaterial>>,
) {
    for (_, shader) in door_highlight_shaders.iter_mut() {
        // TODO: maybe find a way to directly query the entity through a commands object
        match door_is_open.get(shader.door_entity) {
            Ok(_) => shader.is_highlighted = true,
            Err(_) => shader.is_highlighted = false,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[bind_group_data(DoorHighlightMaterialKey)]
struct DoorHighlightMaterial {
    #[uniform(0)]
    fill_color: LinearRgba,
    #[uniform(1)]
    highlight_color: LinearRgba,
    is_highlighted: bool,
    door_entity: Entity,
}

impl DoorHighlightMaterial {
    fn new(entity: Entity) -> Self {
        Self {
            // Blue, full opacity
            fill_color: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            // Yellow, full opacity
            highlight_color: LinearRgba::new(1.0, 1.0, 0.0, 1.0),
            is_highlighted: false,
            door_entity: entity,
        }
    }
}

impl Material2d for DoorHighlightMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        DOOR_HIGHLIGHT_SHADER_PATH.into()
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        key: bevy::sprite_render::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if key.bind_group_data.is_highlighted {
            descriptor
                .fragment
                .as_mut()
                .unwrap()
                .shader_defs
                .push("IS_HIGHLIGHTED".into());
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct DoorHighlightMaterialKey {
    is_highlighted: bool,
}

impl From<&DoorHighlightMaterial> for DoorHighlightMaterialKey {
    fn from(material: &DoorHighlightMaterial) -> Self {
        Self {
            is_highlighted: material.is_highlighted,
        }
    }
}
