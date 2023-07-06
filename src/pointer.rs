use bevy::{prelude::*, window::PrimaryWindow};

use crate::board::*;
use crate::*;
pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeftClickEvent>()
            .add_system(cursor_system)
            .add_system(click_processor.after(cursor_system));
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
fn click_processor(
    mut commands: Commands,
    mut left_click: EventReader<LeftClickEvent>,
    game_board: Res<GameBoard>,
    selected_tile: Option<ResMut<SelectedTile>>,
    swap: Option<ResMut<TileSwap>>,
) {
    if !left_click.is_empty() {
        for event in left_click.iter() {
            match game_board.find_tile(event.position) {
                Some(index) => {
                    let x = index % BOARD_WIDTH;
                    let y = index / BOARD_WIDTH;
                    match &selected_tile {
                        Some(selected) => {
                            let distance = (x.abs_diff(selected.x)) + (y.abs_diff(selected.y));
                            match distance {
                                0 => {
                                    commands.remove_resource::<SelectedTile>();
                                    println!("Deselected Tile: {}, {}", x, y);
                                }
                                1 => {
                                    commands.insert_resource(TileSwap {
                                        tile1: game_board.idx(x, y),
                                        tile2: game_board.idx(selected.x, selected.y),
                                    });
                                    commands.remove_resource::<SelectedTile>();
                                    println!(
                                        "Swapsies! {}, {} and {}, {}",
                                        x, y, selected.x, selected.y
                                    );
                                }
                                _ => {
                                    commands.insert_resource(SelectedTile { x, y });
                                    println!("Changed Selected Tile: {}, {}", x, y);
                                }
                            }
                        }
                        None => {
                            commands.insert_resource(SelectedTile { x, y });
                            println!("Selected New Tile: {}, {}", x, y);
                        }
                    }
                }
                None => {
                    commands.remove_resource::<SelectedTile>();
                    println!("Empty Tile Selected, Deselecting!");
                }
            }

            println!(
                "Someone clicked! World coords: {}/{}",
                event.position.x, event.position.y
            );
        }
    }
}
