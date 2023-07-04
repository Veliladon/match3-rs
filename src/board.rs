use crate::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Resource)]
pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    pub forward: Vec<Option<Entity>>,
    pub backward: HashMap<Entity, usize>,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        return Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            forward: vec![None; BOARD_WIDTH * BOARD_HEIGHT],
            backward: HashMap::new(),
        };
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        return y * self.width + x;
    }
}
