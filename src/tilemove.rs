use crate::*;
use bevy::prelude::*;

pub struct TileMovePlugin;

impl Plugin for TileMovePlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(tile_move_system);
    }
}

/* pub fn tile_move_system(
    mut commands: Commands,
    mut move_query: Query<(Entity, &mut Transform, &mut TileMoving), With<TileMoving>>,
    time: Res<Time>,
    mut game_board: ResMut<GameBoard>,
) {
    for (entity, mut transform, mut tile_move) in move_query.iter_mut() {
        tile_move.duration.tick(time.delta());
        let offsets = game_board.get_offsets();

        if tile_move.duration.just_finished() {
            let index = game_board.idx(tile_move.destination);

            transform.translation.x = tile_move.destination.x as f32 * TILE_WIDTH + offsets.x;
            transform.translation.y = -(tile_move.destination.y as f32 * TILE_HEIGHT + offsets.y);

            game_board.forward[index] = Some(entity);
            game_board.backward.insert(entity, index);
            commands.entity(entity).remove::<TileMoving>();
        } else {

            //transform.translation.x += TILE_WIDTH * destination.duration.percent()
            // transform.translation.y
        }
    }
} */
