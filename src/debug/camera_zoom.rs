use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Resource, Deref, DerefMut)]
struct CameraZoomSensitivity(pub f32);

impl Default for CameraZoomSensitivity {
    fn default() -> Self {
        Self(0.0025)
    }
}

pub struct CameraZoomPlugin;

impl Plugin for CameraZoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraZoomSensitivity::default())
            .add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
pub struct CameraZoom;

fn camera_zoom(
    sensitivity: Res<CameraZoomSensitivity>,
    mut scroll_input: MessageReader<MouseWheel>,
    mut camera: Single<&mut Projection, With<CameraZoom>>,
) {
    for event in scroll_input.read() {
        // TODO: modify this to handle different scroll units
        if let Projection::Orthographic(projection) = camera.as_mut() {
            projection.scale = (projection.scale - event.y * **sensitivity).clamp(0.1, 5.0);
        }
    }
}
