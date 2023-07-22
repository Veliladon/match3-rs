use crate::*;
use bevy::prelude::*;
use simple_easing::expo_in_out;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum MoveState {
    #[default]
    NotMoving,
    Moving,
}

pub struct TileMovePlugin;

impl Plugin for TileMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, tile_mover)
            .add_state::<MoveState>();
    }
}

pub fn tile_mover(
    mut commands: Commands,
    mut move_query: Query<(Entity, &mut Transform, &mut TileMoving), With<TileMoving>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<MoveState>>,
) {
    let mut num_tile_moving = move_query.iter().count();

    if num_tile_moving > 0 {
        next_state.set(MoveState::Moving);
    }

    for (entity, mut transform, mut tile_move) in move_query.iter_mut() {
        // We tick the TileMoving timer along
        tile_move.duration.tick(time.delta());

        // Is the transition finished?
        if tile_move.duration.just_finished() {
            // If the transition is finished we clean up the x and y to precisely match and remove the component
            transform.translation.x = tile_move.destination.x;
            transform.translation.y = tile_move.destination.y;
            commands.entity(entity).remove::<TileMoving>();
            num_tile_moving -= 1;
            info!("Finished Moving Tile");
        } else {
            // Otherwise we update the tile's transform based on an easing function

            let mut final_transform = tile_move.destination - tile_move.origin;
            let percent_complete = tile_move.duration.percent();
            let eased_percent = expo_in_out(percent_complete);
            final_transform = final_transform * eased_percent;
            transform.translation.x = tile_move.origin.x + final_transform.x;
            transform.translation.y = tile_move.origin.y + final_transform.y;
        }
    }
    if num_tile_moving == 0 {
        next_state.set(MoveState::NotMoving);
    }
}
