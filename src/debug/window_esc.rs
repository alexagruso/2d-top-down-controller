use bevy::prelude::*;

pub struct WindowEscapePlugin;

impl Plugin for WindowEscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_esc);
    }
}

fn exit_on_esc(key_input: Res<ButtonInput<KeyCode>>, mut exit_event: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::Escape) {
        exit_event.write(AppExit::Success);
    }
}
