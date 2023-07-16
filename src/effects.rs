use crate::*;
use bevy::prelude::*;
use simple_easing::{expo_in, expo_out, reverse};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_sprite_to_selected_tile);
        app.add_system(animated_selected_tile);
    }
}

pub fn add_sprite_to_selected_tile(
    mut commands: Commands,
    selected_tile: Option<ResMut<SelectedTile>>,
    game_board: Res<GameBoard>,
    mut highlight_query: Query<(Entity, &mut Transform), With<TileHighlight>>,
) {
    match selected_tile {
        Some(mut selected) => {
            let board_entity = game_board.entity;
            let selected_pos = selected.as_mut().as_uvec2();
            let board_pos = game_board.get_board_pos(selected_pos);

            if let Ok((_, transform)) = &mut highlight_query.get_single_mut() {
                transform.translation.x = board_pos.x;
                transform.translation.y = board_pos.y;
            } else {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            board_pos.x,
                            board_pos.y,
                            3.0,
                        )),
                        ..default()
                    })
                    .insert(TileHighlight::default())
                    .insert(TilePosition(selected_pos))
                    .set_parent(board_entity);
            }
        }
        None => {
            if let Ok((entity, _)) = highlight_query.get_single_mut() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn animated_selected_tile(
    mut highlight_query: Query<(&mut Sprite, &mut TileHighlight), With<TileHighlight>>,
    time: Res<Time>,
) {
    if let Ok((mut sprite, mut highlight_timer)) = highlight_query.get_single_mut() {
        highlight_timer.0.tick(time.delta());
        let duration = highlight_timer.0.percent();
        let mut new_alpha = 0.0;

        if duration < 0.5 {
            new_alpha = expo_in((duration % 0.5) * 2.0);
        }
        if duration >= 0.5 {
            new_alpha = expo_out((duration % 0.5) * 2.0);
            new_alpha = reverse(new_alpha);
        }
        sprite.color.set_a(new_alpha);
        //println!("Duration: {}, New Alpha: {}", duration, new_alpha);
    }
}
