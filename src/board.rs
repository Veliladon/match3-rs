use crate::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Resource)]
pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    pub forward: Vec<Option<Entity>>,
    pub backward: HashMap<Entity, usize>,
    pub origin: Vec2,
}

impl GameBoard {
    pub fn new(windowsize: Vec2) -> GameBoard {
        let origin = find_origin(windowsize);

        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            forward: vec![None; BOARD_WIDTH * BOARD_HEIGHT],
            backward: HashMap::new(),
            origin: origin,
        }
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        return y * self.width + x;
    }
}

pub fn find_origin(windowsize: Vec2) -> Vec2 {
    let window_height = windowsize.y;
    let window_width = windowsize.x;
    let board_height = BOARD_HEIGHT as f32 * SPRITE_SCALE * TILE_HEIGHT;
    let board_width = BOARD_WIDTH as f32 * SPRITE_SCALE * TILE_WIDTH;

    let top_margin = (window_height / 2.0) - (board_height / 2.0);
    println!("Top Margin: {}", top_margin);

    let top_left_x = (window_width / 2.0) - (top_margin + board_width);
    let top_left_y = -(board_height / 2.0);

    println!("Board top_left: {}, {}", top_left_x, top_left_y);
    Vec2::new(top_left_x, top_left_y)
}
