use crate::*;
use bevy::prelude::*;
use simple_easing::expo_in_out;

pub struct TileMovePlugin;

impl Plugin for TileMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tile_mover);
    }
}

pub fn tile_mover(
    mut commands: Commands,
    mut move_query: Query<(Entity, &mut Transform, &mut TileMoving), With<TileMoving>>,
    time: Res<Time>,
    game_board: Res<GameBoard>,
) {
    for (entity, mut transform, mut tile_move) in move_query.iter_mut() {
        // We tick the TileMoving timer along
        tile_move.duration.tick(time.delta());

        // We need the detination in board space whether the timer is still going or finished
        let dest_transform = game_board.find_local_from_grid(tile_move.destination);

        // Is the transition finished?
        if tile_move.duration.just_finished() {
            // If the transition is finished we clean up the x and y to precisely match and remove the component
            transform.translation.x = dest_transform.x;
            transform.translation.y = dest_transform.y;
            commands.entity(entity).remove::<TileMoving>();
        } else {
            // Otherwise we update the tile's transform based on an easing function
            let origin_transform = game_board.find_local_from_grid(tile_move.origin);
            let mut final_transform = dest_transform - origin_transform;
            let percent_complete = tile_move.duration.percent();
            let eased_percent = expo_in_out(percent_complete);
            final_transform = final_transform * eased_percent;
            transform.translation.x = origin_transform.x + final_transform.x;
            transform.translation.y = origin_transform.y + final_transform.y;
        }
    }
}
