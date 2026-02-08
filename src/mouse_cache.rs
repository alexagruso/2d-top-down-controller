#![allow(dead_code, unused_variables)]

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default)]
struct MouseCache {
    window_position: Option<Vec2>,
    world_position: Option<Vec2>,
}

struct MouseCachePlugin;

impl Plugin for MouseCachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCache>()
            // TODO: revisit this schedule placement
            .add_systems(PreUpdate, cache_cursor_position);
    }
}

fn cache_cursor_position(
    window: Single<&Window, With<PrimaryWindow>>,
    mut cursor_cache: ResMut<MouseCache>,
) {
    match window.cursor_position() {
        Some(position) => {}
        None => {
            cursor_cache.window_position = None;
        }
    }
}
