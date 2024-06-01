use crate::*;
use bevy::prelude::*;
use simple_easing::{expo_in, expo_out, reverse};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_sprite_to_selected_tile);
        app.add_systems(Update, animated_selected_tile);
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
            if selected.is_changed() || selected.is_added() {
                if let Ok((entity, _)) = highlight_query.get_single_mut() {
                    commands.entity(entity).despawn();
                }
                let selected_pos = selected.as_mut().as_uvec2();
                let selected_idx = game_board.idx(selected_pos);
                let selected_entity = game_board.backward.get(&selected_idx).copied().unwrap();
                commands.entity(selected_entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                                custom_size: Some(Vec2::new(SHEET_TILE_WIDTH, SHEET_TILE_HEIGHT)),

                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 3.0),
                                scale: Vec3::splat(1.0),
                                ..Default::default()
                            },

                            ..default()
                        })
                        .insert(TileHighlight::default())
                        .insert(TilePosition(selected_pos));
                });
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
        let duration = highlight_timer.0.fraction();
        let mut new_alpha = 0.0;

        if duration < 0.5 {
            new_alpha = expo_in((duration % 0.5) * 2.0);
        }
        if duration >= 0.5 {
            new_alpha = expo_out((duration % 0.5) * 2.0);
            new_alpha = reverse(new_alpha);
        }
        sprite.color.set_a(new_alpha);
    }
}
