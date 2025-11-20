use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    debug::CameraZoom,
    math::{rotate_vec2_radians, window_to_viewport_position},
};

#[derive(Component)]
pub struct ViewCone {
    radius: f32,
    // In radians
    view_angle: f32,
    maximum_ray_spacing: f32,
}

impl ViewCone {
    pub fn new(radius: f32, view_angle: f32) -> Self {
        Self {
            radius,
            view_angle,
            maximum_ray_spacing: f32::to_radians(5.0),
        }
    }

    pub fn with_minimum_ray_spacing(self, maximum_ray_spacing: f32) -> Self {
        Self {
            maximum_ray_spacing,
            ..self
        }
    }
}

pub struct ViewConePlugin;

impl Plugin for ViewConePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_view_cones);
    }
}

fn update_view_cones(
    camera: Single<&Projection, With<CameraZoom>>,
    viewing_entities: Query<(&ViewCone, &Transform, &CollisionLayers)>,
    window: Single<&Window, With<PrimaryWindow>>,
    spatial_query: Res<SpatialQueryPipeline>,
    mut gizmos: Gizmos,
) {
    // TODO: extract this into a general resource
    let projection = if let Projection::Orthographic(projection) = *camera {
        projection
    } else {
        // TODO: maybe handle other camera projections
        return;
    };

    let cursor_offset = match window.cursor_position() {
        Some(position) => window_to_viewport_position(position, window.size()),
        // TODO: make this cache the cursor position so the laser can be drawn even if the cursor
        // is outside of the window
        None => return,
    } * projection.scale;

    for (view_cone, transform, layers) in &viewing_entities {
        let rays = (view_cone.view_angle / view_cone.maximum_ray_spacing).ceil() as usize;
        let ray_spacing = view_cone.view_angle / rays as f32;
        let initial_angle = view_cone.view_angle / 2.0;

        let start = transform.translation.xy();

        for i in 0..rays {
            let angle = -initial_angle + ray_spacing * i as f32;
            let ray = rotate_vec2_radians(cursor_offset, angle).normalize();
            let direction = Dir2::new(ray).unwrap();

            let length = if let Some(hit) = spatial_query.cast_ray(
                start,
                direction,
                view_cone.radius,
                true,
                &SpatialQueryFilter::from_mask(layers.filters),
            ) {
                hit.distance
            } else {
                view_cone.radius
            };
            let ray = (ray * length).clamp_length_max(view_cone.radius);

            gizmos.ray_2d(start, ray, Color::srgb(1.0, 0.0, 0.0));
            // gizmos.circle_2d(start + ray, 5.0, Color::srgb(1.0, 0.0, 0.0));
        }
    }
}
