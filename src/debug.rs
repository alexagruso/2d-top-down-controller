use avian2d::prelude::{Physics, PhysicsTime};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

mod camera_zoom;
mod fps_overlay;
mod window_esc;

pub use camera_zoom::*;
pub use fps_overlay::*;
pub use window_esc::*;

pub struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraZoomPlugin,
            FpsOverlayPlugin::new(Color::srgb(1.0, 1.0, 0.0)),
            WindowEscapePlugin,
            FrameStepPlugin,
        ));
    }
}

struct FrameStepPlugin;

impl Plugin for FrameStepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_paused.run_if(input_just_pressed(KeyCode::KeyP)),
                step.run_if(input_just_pressed(KeyCode::Enter)),
            ),
        );
    }
}

fn toggle_paused(mut time: ResMut<Time<Physics>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

fn step(mut physics_time: ResMut<Time<Physics>>, fixed_time: Res<Time<Fixed>>) {
    physics_time.advance_by(fixed_time.delta());
}
