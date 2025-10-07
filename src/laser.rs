use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::characters::Player;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_laser);
    }
}

fn update_laser(
    player: Single<(&Transform, &CollisionLayers), With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut gizmos: Gizmos,
) {
    let (transform, collision_layers) = player.into_inner();

    let start = transform.translation.xy();
    let end = match window.cursor_position() {
        Some(position) => cursor_to_camera_position(position, window.size()),
        None => return,
    } + start;

    let mut ray = end - start;
    let direction = match Dir2::new(ray) {
        Ok(result) => result,
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
        ray_color = Color::srgb(1.0, 0.0, 0.0);
    }

    gizmos.ray_2d(start, ray, ray_color);
}

fn cursor_to_camera_position(cursor_position: Vec2, window_size: Vec2) -> Vec2 {
    Vec2::new(
        cursor_position.x - window_size.x / 2.0,
        -cursor_position.y + window_size.y / 2.0,
    )
}
