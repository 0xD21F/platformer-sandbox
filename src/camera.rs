use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;

const ASPECT_RATIO: f32 = 16. / 9.;

pub fn fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<Camera2d>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok((mut orthographic_projection, mut camera_transform)) = camera_query.get_single_mut() {
        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    // If the level we're checking is the current active level, then we want to use the level's dimensions to determine the camera's bounds.
                    // NOTE: The first parameter does nothing for is_match using a LevelSelection::Identifier enum, but is a required parameter to the function.
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;
                    orthographic_projection.scaling_mode = ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;

                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (10.
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (10.
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                } else {
                    // level.visibility = N
                }
            }
        }
    }
}
