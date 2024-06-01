use bevy::{prelude::*, window::PrimaryWindow};

use crate::board::*;
use crate::distance::LDistance;
use crate::*;
pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeftClickEvent>()
            .add_systems(Update, cursor_system)
            .add_systems(Update, click_processor.after(cursor_system));
    }
}
#[derive(Event)]
pub struct LeftClickEvent {
    pub position: Vec2,
}

fn cursor_system(
    btn: Res<ButtonInput<MouseButton>>,
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

            info!(
                "Clicked! World coords: {}/{}",
                world_position.x, world_position.y
            );
        }
    }
}
fn click_processor(
    mut commands: Commands,
    mut left_click: EventReader<LeftClickEvent>,
    mut game_board: ResMut<GameBoard>,
    selected_tile: Option<ResMut<SelectedTile>>,
) {
    if !left_click.is_empty() {
        let selected_pos = selected_tile.map(|x| x.as_uvec2());

        for event in left_click.read() {
            match game_board.find_grid_from_world(event.position) {
                Some(grid_pos) => match selected_pos {
                    Some(selected_pos) => {
                        let distance = grid_pos.ldistance(selected_pos);
                        match distance {
                            0 => {
                                commands.remove_resource::<SelectedTile>();

                                info!("Deselected Tile: {}, {}", grid_pos.x, grid_pos.y);
                            }
                            1 => {
                                let tile1_index = game_board.idx(grid_pos);
                                let tile2_index = game_board.idx(selected_pos);

                                let tile1 = game_board.forward.get(tile1_index).copied().unwrap();
                                let tile2 = game_board.forward.get(tile2_index).copied().unwrap();

                                let tile1_entity = game_board.get_entity(grid_pos).unwrap();
                                let tile2_entity = game_board.get_entity(selected_pos).unwrap();

                                game_board.forward[tile2_index] = tile1;
                                game_board.forward[tile1_index] = tile2;

                                game_board.backward.insert(tile2_index, tile1_entity);
                                game_board.backward.insert(tile1_index, tile2_entity);

                                let grid_transform = game_board.find_local_from_grid(grid_pos);
                                let selected_transform =
                                    game_board.find_local_from_grid(selected_pos);

                                commands.entity(tile1_entity).insert(TileMoving {
                                    origin: grid_transform,
                                    destination: selected_transform,

                                    duration: Timer::from_seconds(0.5, TimerMode::Once),
                                });
                                commands.entity(tile2_entity).insert(TileMoving {
                                    origin: selected_transform,
                                    destination: grid_transform,
                                    duration: Timer::from_seconds(0.5, TimerMode::Once),
                                });

                                commands.remove_resource::<SelectedTile>();

                                info!(
                                    "Swapsies! {}, {} and {}, {}",
                                    grid_pos.x, grid_pos.y, selected_pos.x, selected_pos.y
                                );
                            }
                            _ => {
                                commands.insert_resource(SelectedTile(grid_pos));

                                info!("Changed Selected Tile: {}, {}", grid_pos.x, grid_pos.y);
                            }
                        }
                    }
                    None => {
                        commands.insert_resource(SelectedTile(grid_pos));

                        info!("Selected New Tile: {}, {}", grid_pos.x, grid_pos.y);
                    }
                },
                None => {
                    commands.remove_resource::<SelectedTile>();

                    info!("Empty Tile Selected, Deselecting!");
                }
            }

            info!(
                "Someone clicked! World coords: {}/{}",
                event.position.x, event.position.y
            );
        }
    }
}
