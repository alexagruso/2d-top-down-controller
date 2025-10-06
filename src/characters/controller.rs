// TODO: change naming convention to character rather than player so that we can semantically apply
// this code to non-player entities.
// TODO: look into changing manual velocity/position movement to avian2d
// linearvelocity/angularvelocity components
// NOTE: link to paper outlining possible improvements to this algorithm:
// https://arxiv.org/ftp/arxiv/papers/1211/1211.0059.pdf

use avian2d::prelude::*;
use bevy::{math::InvalidDirectionError, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, character_collision_response);
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

// FIX: occasional jittering when moving a flat surface against a corner which is being caused by
// the
// HACK: limiting the bounces to 2 prevents the double-surface jittering issue, but I'd like to
// remove this limitation to handle arbitrarily detailed geometry
fn character_collision_response(
    time: Res<Time<Fixed>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut player_query: Query<(&mut Transform, &Velocity, &Collider, &CollisionLayers)>,
) {
    let (mut transform, velocity, collider, collision_layers) = player_query.single_mut().unwrap();
    let player_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
    let player_angle_unit = vec2(player_angle.cos(), player_angle.sin());

    let mut cast_origin = transform.translation.xy();
    let mut cast_velocity = velocity.0 * time.delta_secs();
    // NOTE: not really necessary for top-down controllers, but if I ever modify this to be a
    // 2d platformer controller, we will need to initialize this as the player velocity and then
    // modify it only if we collide with something.
    let mut delta_velocity = Vec2::ZERO;

    let config = CollideAndSlideConfig::default();

    // TODO: extract this into a separate function
    'bounces: for _ in 0..config.bounces {
        let direction = match Dir2::new(cast_velocity) {
            Ok(result) => result,
            // HACK: If the velocity is zero, we set some dummy direction to satisfy the function
            // call. Maybe we don't need to do this; the spatial query pipeline object might have a
            // better function for this (i don't think it does)
            Err(InvalidDirectionError::Zero) => Dir2::X,
            Err(_) => panic!("cast velocity is either infinite or NaN"),
        };

        if let Some(hit) = spatial_query.cast_shape(
            &collider,
            cast_origin,
            player_angle,
            direction,
            &ShapeCastConfig {
                max_distance: cast_velocity.length() + config.skin_width,
                ..default()
            },
            &SpatialQueryFilter::from_mask(collision_layers.filters),
        ) {
            // Maximum distance the collider can move in the direction of the cast without
            // hitting another entity
            let snap_to_surface =
                cast_velocity.normalize_or_zero() * (hit.distance - config.skin_width).max(0.0);

            // If the hit distance is 0 the shapes are colliding and we need to handle it
            // separately
            if hit.distance > 0.0 {
                // Move collider as far as we can in the direction of the cast and calculate
                // remainder velocity for the next step
                delta_velocity += snap_to_surface;
                cast_origin += snap_to_surface;
                cast_velocity = (cast_velocity - snap_to_surface).reject_from(hit.normal1);
            } else {
                // Push the player out by the penetration depth
                // TODO: this isn't a perfect implementation; minor jittering when a flat surface
                // moves along a corner and inconsistent movement when the collider is moving
                // along a flat surface while rotating.
                let world_hit = hit.point1;
                let character_hit =
                    hit.point2.rotate(player_angle_unit) + transform.translation.xy();
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
