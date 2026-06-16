use avian2d::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*, winit::WinitPlugin,
};
use topdown_controller_2d::{GamePlugin, debug::GameDebugPlugin};

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Burn Notice"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WinitPlugin::default()),
            FrameTimeDiagnosticsPlugin::default(),
            PhysicsPlugins::default().with_length_unit(20.0),
            GamePlugin,
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            #[cfg(debug_assertions)]
            GameDebugPlugin,
        ))
        .run();
}
