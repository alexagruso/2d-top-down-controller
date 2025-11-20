// NOTE: link to paper outlining possible improvements to this algorithm that would fix the
// jittering when flat surfaces move along sharp corners:
// https://arxiv.org/ftp/arxiv/papers/1211/1211.0059.pdf

use avian2d::prelude::*;
use bevy::{math::InvalidDirectionError, prelude::*};

#[derive(Component)]
#[require(
    Transform,
    // BUG: using ['LinearVelocity'] causes a one frame delay between the player moving and the
    // mesh visually updating
    LinearVelocity,
    // We use ['RigidBody::Kinematic'] since collision responses for all controllers are handled
    // manually.
    RigidBody::Kinematic,
    Collider,
    CollisionLayers
)]
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

#[derive(Clone, Copy)]
pub enum MovementType {
    Translation(Vec2),
    Rotation(f32),
}

// TODO: look into making this use `EntityEvent` or observers rather than `Message`
#[derive(Message)]
pub struct ControllerMovement {
    movement: MovementType,
    entity: Entity,
}

impl ControllerMovement {
    pub fn from_translation(translation: Vec2, entity: Entity) -> Self {
        Self {
            movement: MovementType::Translation(translation),
            entity,
        }
    }

    pub fn from_rotation(angle: f32, entity: Entity) -> Self {
        Self {
            movement: MovementType::Rotation(angle),
            entity,
        }
    }
}

fn controller_movement(
    mut controllers: Query<(&mut Transform, &mut LinearVelocity), With<CharacterController>>,
    mut movement_messages: MessageReader<ControllerMovement>,
) {
    for event in movement_messages.read() {
        let (mut transform, mut velocity) = match controllers.get_mut(event.entity) {
            Ok(result) => result,
            Err(_) => {
                unreachable!(
                    "Entities with a `CharacterController` require a transform and velocity"
                )
            }
        };

        match event.movement {
            MovementType::Translation(delta_velocity) => velocity.0 = delta_velocity,
            MovementType::Rotation(angle) => transform.rotate_z(angle),
        }
    }
}

#[derive(Clone)]
struct CollideAndSlideData<'a> {
    transform: Transform,
    initial_velocity: LinearVelocity,
    collider: &'a Collider,
    collision_layers: CollisionLayers,
    fixed_delta_time: f32,
}

#[derive(Clone, Copy)]
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

fn collide_and_slide(
    data: &CollideAndSlideData,
    config: &CollideAndSlideConfig,
    spatial_query_pipeline: &SpatialQueryPipeline,
) -> Vec2 {
    // Z-component of the XYZ rotation of the object
    let collider_angle = data.transform.rotation.to_euler(EulerRot::XYZ).2;
    let angle_unit_vector = vec2(collider_angle.cos(), collider_angle.sin());

    let mut cast_origin = data.transform.translation.xy();
    let mut cast_velocity = data.initial_velocity.0 * data.fixed_delta_time;
    let mut result_velocity = Vec2::ZERO;

    'bounces: for _ in 0..config.bounces {
        let direction = match Dir2::new(cast_velocity) {
            Ok(result) => result,
            // HACK: If the velocity is zero, we set some dummy direction to satisfy the function
            // call. Maybe we don't need to do this; the spatial query pipeline API might have a
            // better function for this (I don't think it does).
            Err(InvalidDirectionError::Zero) => Dir2::X,
            Err(_) => panic!("cast velocity is either infinite or NaN"),
        };

        if let Some(hit) = spatial_query_pipeline.cast_shape(
            data.collider,
            cast_origin,
            collider_angle,
            direction,
            &ShapeCastConfig {
                max_distance: cast_velocity.length() + config.skin_width,
                ..default()
            },
            &SpatialQueryFilter::from_mask(data.collision_layers.filters),
        ) {
            // Maximum distance the collider can move in the direction of the cast without
            // hitting another entity.
            let snap_to_surface =
                cast_velocity.normalize_or_zero() * (hit.distance - config.skin_width).max(0.0);

            if hit.distance > 0.0 {
                // First, we move the collider as far as we can in the direction of the cast.
                // Next, we reject the remaining velocity from the hit normal to get the new cast
                // velocity (which is parallel to the surface that was hit).
                result_velocity += snap_to_surface;
                cast_origin += snap_to_surface;
                cast_velocity = (cast_velocity - snap_to_surface).reject_from(hit.normal1);
            } else {
                // If the hit distance is 0.0 the shapes are colliding and we need to push the
                // collider out by the penetration depth.
                let world_hit = hit.point1;
                let character_hit =
                    hit.point2.rotate(angle_unit_vector) + data.transform.translation.xy();
                result_velocity += world_hit - character_hit;
            }
        } else {
            // No collision was detected, so we move the remaining distance and break the loop.
            result_velocity += cast_velocity;
            break 'bounces;
        }
    }

    result_velocity
}

fn controller_collision_response(
    time: Res<Time<Fixed>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut controllers: Query<(&mut LinearVelocity, &Transform, &Collider, &CollisionLayers)>,
) {
    for (mut velocity, transform, collider, layers) in &mut controllers {
        let fixed_delta_time = time.delta_secs();
        let data = CollideAndSlideData {
            transform: *transform,
            initial_velocity: *velocity,
            collider,
            collision_layers: *layers,
            fixed_delta_time,
        };
        let config = CollideAndSlideConfig::default();

        let result_velocity = collide_and_slide(&data, &config, &spatial_query);
        // The result velocity is raw, and so we need to scale back up by delta time to work with
        // avian's [`LinearVelocity`] component
        **velocity = result_velocity / fixed_delta_time;
    }
}
