use bevy::{prelude::*, window::PrimaryWindow};

use crate::*;
pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeftClickEvent>().add_system(cursor_system);
    }
}

pub struct LeftClickEvent {
    pub position: Vec2,
}

fn cursor_system(
    btn: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut left_click: EventWriter<LeftClickEvent>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.get_single().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            left_click.send(LeftClickEvent {
                position: (world_position.x, world_position.y).into(),
            });
            println!(
                "Clicked! World coords: {}/{}",
                world_position.x, world_position.y
            );
        }
    }
}
