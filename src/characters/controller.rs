// TODO: look into changing manual velocity/position movement to avian2d
// linearvelocity/angularvelocity components
// NOTE: link to paper outlining possible improvements to this algorithm:
// https://arxiv.org/ftp/arxiv/papers/1211/1211.0059.pdf

use avian2d::prelude::*;
use bevy::{math::InvalidDirectionError, prelude::*};

#[derive(Component)]
#[require(Transform, Velocity, Collider, CollisionLayers)]
pub struct CharacterController;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            (controller_movement, controller_collision_response).chain(),
        );
    }
}

pub enum Movement {
    Translation(Vec2),
    Rotation(f32),
}

// TODO: look into making this use `EntityEvent` rather than `Message`
#[derive(Message)]
pub struct ControllerMovement {
    movement: Movement,
    entity: Entity,
}

impl ControllerMovement {
    pub fn from_translation(translation: Vec2, entity: Entity) -> Self {
        Self {
            movement: Movement::Translation(translation),
            entity,
        }
    }

    pub fn from_rotation(angle: f32, entity: Entity) -> Self {
        Self {
            movement: Movement::Rotation(angle),
            entity,
        }
    }
}

fn controller_movement(
    mut controllers: Query<(&mut Transform, &mut Velocity), With<CharacterController>>,
    mut movement_messages: MessageReader<ControllerMovement>,
) {
    for event in movement_messages.read() {
        // PERF: This may be unnecessarily called multiple times per frame if one entity has
        // multiple movement messages.
        // Related to the TODO above the ControllerMovement struct
        let (mut transform, mut velocity) = match controllers.get_mut(event.entity) {
            Ok(result) => result,
            Err(_) => {
                unreachable!(
                    "Entities with a `CharacterController` require a transform and velocity"
                )
            }
        };

        match event.movement {
            Movement::Translation(delta_velocity) => velocity.0 = delta_velocity,
            Movement::Rotation(angle) => transform.rotate_z(angle),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
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

fn controller_collision_response(
    time: Res<Time<Fixed>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut characters: Query<(&mut Transform, &Velocity, &Collider, &CollisionLayers)>,
) {
    for (mut transform, velocity, collider, collision_layers) in &mut characters {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let angle_unit_vector = vec2(angle.cos(), angle.sin());

        let mut cast_origin = transform.translation.xy();
        let mut cast_velocity = velocity.0 * time.delta_secs();
        let mut delta_velocity = Vec2::ZERO;

        let config = CollideAndSlideConfig::default();

        // TODO: extract this into a separate function
        'bounces: for _ in 0..config.bounces {
            let direction = match Dir2::new(cast_velocity) {
                Ok(result) => result,
                // HACK: If the velocity is zero, we set some dummy direction to satisfy the function
                // call. Maybe we don't need to do this; the spatial query pipeline API might have a
                // better function for this (I don't think it does).
                Err(InvalidDirectionError::Zero) => Dir2::X,
                Err(_) => panic!("cast velocity is either infinite or NaN"),
            };

            if let Some(hit) = spatial_query.cast_shape(
                &collider,
                cast_origin,
                angle,
                direction,
                &ShapeCastConfig {
                    max_distance: cast_velocity.length() + config.skin_width,
                    ..default()
                },
                &SpatialQueryFilter::from_mask(collision_layers.filters),
            ) {
                // Maximum distance the collider can move in the direction of the cast without
                // hitting another entity.
                let snap_to_surface =
                    cast_velocity.normalize_or_zero() * (hit.distance - config.skin_width).max(0.0);

                // If the hit distance is 0 the shapes are colliding and we need to handle it
                // separately.
                if hit.distance > 0.0 {
                    // Move collider as far as we can in the direction of the cast and calculate
                    // remainder velocity for the next step.
                    delta_velocity += snap_to_surface;
                    cast_origin += snap_to_surface;
                    cast_velocity = (cast_velocity - snap_to_surface).reject_from(hit.normal1);
                } else {
                    // Push the collider out by the penetration depth.
                    let world_hit = hit.point1;
                    let character_hit =
                        hit.point2.rotate(angle_unit_vector) + transform.translation.xy();
                    delta_velocity += world_hit - character_hit;
                }
            } else {
                // No collision was detected, so we move the remaining distance and break the loop.
                delta_velocity += cast_velocity;
                break 'bounces;
            }
        }

        transform.translation += delta_velocity.extend(0.0);
    }
}
