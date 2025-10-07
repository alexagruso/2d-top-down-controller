use bevy::prelude::*;

mod camera_zoom;
mod fps_overlay;
mod window_esc;

pub use camera_zoom::*;
pub use fps_overlay::*;
pub use window_esc::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraZoomPlugin,
            FpsOverlayPlugin::new(Color::srgb(1.0, 1.0, 0.0)),
            WindowEscapePlugin,
        ));
    }
}
