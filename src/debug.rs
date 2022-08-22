use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_system(debug_camera);
        }
    }
}

pub fn debug_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    keys: Res<Input<KeyCode>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if keys.pressed(KeyCode::LControl) {
            if keys.pressed(KeyCode::Up) {
                camera_transform.scale.x += 0.1;
                camera_transform.scale.y += 0.1;
            }
            if keys.pressed(KeyCode::Down) {
                if camera_transform.scale.x > 0.1 && camera_transform.scale.y > 0.1 {
                    camera_transform.scale.x -= 0.1;
                    camera_transform.scale.y -= 0.1;
                }
            }
        } else {
            let mut multiplier = 1.0;
            if keys.pressed(KeyCode::LShift) {
                multiplier = 10.0;
            }
            if keys.pressed(KeyCode::Right) {
                camera_transform.translation.x += 5. * multiplier;
            }
            if keys.pressed(KeyCode::Left) {
                camera_transform.translation.x -= 5. * multiplier;
            }
            if keys.pressed(KeyCode::Up) {
                camera_transform.translation.y += 5. * multiplier;
            }
            if keys.pressed(KeyCode::Down) {
                camera_transform.translation.y -= 5. * multiplier;
            }
        }
    }
}
