use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraZoomPlugin;

impl Plugin for CameraZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
pub struct CameraZoom;

fn camera_zoom(
    mut scroll_input: MessageReader<MouseWheel>,
    mut camera: Query<&mut Projection, With<CameraZoom>>,
) {
    let mut projection = match camera.single_mut() {
        Ok(result) => result,
        Err(_) => panic!("There cannot be more than one zoomable camera."),
    };

    for event in scroll_input.read() {
        // TODO: modify this to handle different scroll units
        if let Projection::Orthographic(projection) = projection.as_mut() {
            projection.scale = (projection.scale - event.y * 0.3).clamp(0.1, 5.0);
        }
    }
}
