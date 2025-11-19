use avian2d::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*};
use topdown_controller_2d::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins((
            // Prevents non-error bevy engine logs from printing to the console.
            DefaultPlugins.build().disable::<LogPlugin>(),
            FrameTimeDiagnosticsPlugin::default(),
            PhysicsPlugins::default().with_length_unit(200.0),
            // PhysicsDebugPlugin::default(),
            GamePlugin,
        ))
        .run();
}
