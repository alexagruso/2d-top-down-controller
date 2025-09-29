use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsOverlayPlugin {
    color: Color,
}

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps_overlay)
            .add_systems(Update, update_fps_overlay);
    }
}

impl Default for FpsOverlayPlugin {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
        }
    }
}

#[derive(Component)]
struct FpsOverlay;

fn setup_fps_overlay(mut commands: Commands) {
    // TODO: make this use text sections
    commands.spawn((
        FpsOverlay,
        Text::new("Fps: "),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

fn update_fps_overlay(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text: Query<&mut Text, With<FpsOverlay>>,
) {
    let mut text = match fps_text.single_mut() {
        Ok(result) => result,
        Err(_) => panic!("There cannot be more than one fps overlay"),
    };

    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        **text = format!("Fps: {fps:.0}");
    }
}
