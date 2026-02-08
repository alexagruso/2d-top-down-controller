#![allow(dead_code, unused_variables)]

// TODO: rename this to `sector`
use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::debug::CameraZoom;

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
) {
}
