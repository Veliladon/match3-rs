use crate::*;

#[derive(Resource)]
pub struct GameBoard {
    width: usize,
    height: usize,
    board: Vec<tile::Tile>,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let board = vec![tile::Tile::new(); BOARD_WIDTH * BOARD_HEIGHT];
        return Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            board: board,
        };
    }
}
