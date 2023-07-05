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
    selected_tile: Option<Res<SelectedTile>>,
    game_board: Res<GameBoard>,
    mut highlight_query: Query<(Entity, &mut Transform), With<TileHighlight>>,
) {
    let x_offset = HALF_TILE_WIDTH + game_board.origin.x;
    let y_offset = HALF_TILE_HEIGHT + game_board.origin.y;

    match &selected_tile {
        Some(selected) => {
            if highlight_query.is_empty() {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            selected.x as f32 * TILE_WIDTH + x_offset,
                            -(selected.y as f32 * TILE_HEIGHT + y_offset),
                            3.0,
                        )),
                        ..default()
                    })
                    .insert(TileHighlight::default())
                    .insert(TilePosition {
                        x: selected.x,
                        y: selected.y,
                    });
            } else {
                let transform = &mut highlight_query.get_single_mut().unwrap().1;
                transform.translation.x = selected.x as f32 * TILE_WIDTH + x_offset;
                transform.translation.y = -(selected.y as f32 * TILE_HEIGHT + y_offset);
            }
        }
        None => {
            if !highlight_query.is_empty() {
                let entity = highlight_query.get_single().unwrap().0;
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
