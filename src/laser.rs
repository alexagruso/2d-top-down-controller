use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{characters::Player, debug::CameraZoom};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_laser);
    }
}

fn update_laser(
    camera: Single<&Projection, With<CameraZoom>>,
    player: Single<(&Transform, &CollisionLayers), With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut gizmos: Gizmos,
) {
    let projection = if let Projection::Orthographic(projection) = *camera {
        projection
    } else {
        // TODO: maybe handle other camera projections
        return;
    };

    let (transform, collision_layers) = player.into_inner();

    let start = transform.translation.xy();
    let end = match window.cursor_position() {
        Some(position) => cursor_to_camera_position(position, window.size()),
        // TODO: make this cache the cursor position so the laser can be drawn even if the cursor
        // is outside of the window
        None => return,
    } + start;

    let mut ray = (end - start) * projection.scale;
    let direction = match Dir2::new(ray) {
        Ok(result) => result,
        // Mouse is directly over the center of the player, don't draw the laser
        Err(_) => return,
    };

    let mut ray_color = Color::srgb(0.0, 1.0, 0.0);

    if let Some(hit) = spatial_query.cast_ray(
        start,
        direction,
        ray.length(),
        true,
        &SpatialQueryFilter::from_mask(collision_layers.filters),
    ) {
        ray = ray.clamp_length_max(hit.distance);

        // TODO: move this to a system in the Update schedule that sends a message or updates a
        // resource
        if mouse_button.pressed(MouseButton::Left) {
            ray_color = Color::srgb(0.0, 0.0, 1.0);
        } else {
            ray_color = Color::srgb(1.0, 0.0, 0.0);
        }
    }

    gizmos.ray_2d(start, ray, ray_color);
    gizmos.circle_2d(start + ray, 5.0, ray_color);
}

fn cursor_to_camera_position(cursor_position: Vec2, window_size: Vec2) -> Vec2 {
    Vec2::new(
        cursor_position.x - window_size.x / 2.0,
        -cursor_position.y + window_size.y / 2.0,
    )
}
