use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Resource, Deref, DerefMut)]
pub struct FpsOverlayColor(pub Color);

pub struct FpsOverlayPlugin {
    color: Color,
}

impl FpsOverlayPlugin {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FpsOverlayColor(self.color))
            .add_systems(Startup, setup_fps_overlay)
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

fn setup_fps_overlay(text_color: Res<FpsOverlayColor>, mut commands: Commands) {
    // TODO: make this use text sections
    commands.spawn((
        FpsOverlay,
        Text::new("Fps: "),
        TextColor(**text_color),
        TextLayout::new_with_justify(Justify::Right),
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
