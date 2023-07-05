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

    pub fn find_tile(&self, position: Vec2) -> Option<usize> {
        let tile_width = TILE_WIDTH * SPRITE_SCALE;
        let tile_height = TILE_HEIGHT * SPRITE_SCALE;
        let board_width = tile_width * BOARD_WIDTH as f32;
        let board_height = tile_height * BOARD_HEIGHT as f32;
        let grid_x = position.x - self.origin.x;
        let grid_y = -position.y - self.origin.y;

        if grid_x < 0.0 || grid_x > board_width {
            return None;
        }
        if grid_y < 0.0 || grid_y > board_height {
            return None;
        }

        println!("Looking for tile in {}, {}", grid_x, grid_y);
        let x = (grid_x / tile_width) as usize;
        let y = (grid_y / tile_height) as usize;
        println!("x: {}, y: {}", x, y);

        Some(self.idx(x, y))
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
