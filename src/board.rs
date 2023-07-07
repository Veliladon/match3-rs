use crate::*;
use bevy::math::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Resource)]
pub struct GameBoard {
    pub dimensions: UVec2,
    pub forward: Vec<Option<Entity>>,
    pub backward: HashMap<Entity, usize>,
    pub origin: Vec2,
}

impl GameBoard {
    pub fn new(dimensions: UVec2, windowsize: Vec2) -> GameBoard {
        let origin = find_origin(windowsize);

        Self {
            dimensions: dimensions,
            forward: vec![None; (dimensions.x * dimensions.y) as usize],
            backward: HashMap::new(),
            origin: origin,
        }
    }

    pub fn idx(&self, grid_pos: UVec2) -> usize {
        return grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize;
    }

    pub fn find_tile(&self, position: Vec2) -> Option<usize> {
        let board_width = TILE_WIDTH * self.dimensions.x as f32;
        let board_height = TILE_HEIGHT * self.dimensions.y as f32;
        let grid_x = position.x - self.origin.x;
        let grid_y = -position.y - self.origin.y;

        if grid_x < 0.0 || grid_x > board_width {
            return None;
        }
        if grid_y < 0.0 || grid_y > board_height {
            return None;
        }

        println!("Looking for tile in {}, {}", grid_x, grid_y);

        let grid_pos = UVec2::new((grid_x / TILE_WIDTH) as u32, (grid_y / TILE_HEIGHT) as u32);

        println!("x: {}, y: {}", grid_pos.x, grid_pos.y);

        Some(self.idx(grid_pos))
    }

    pub fn find_grid(&self, index: usize) -> UVec2 {
        UVec2::new(
            (index % self.dimensions.x as usize) as u32,
            (index / self.dimensions.y as usize) as u32,
        )
    }

    pub fn get_tile(&self, grid_pos: UVec2) -> Option<Entity> {
        return self.forward
            [grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize];
    }

    pub fn get_world_pos(&self, grid_pos: UVec2) -> Vec2 {
        let offsets = self.get_offsets();
        Vec2::new(
            grid_pos.x as f32 * TILE_WIDTH + offsets.x,
            -(grid_pos.y as f32 * TILE_HEIGHT + offsets.y),
        )
    }

    pub fn get_offsets(&self) -> Vec2 {
        let x_offset = HALF_TILE_WIDTH + self.origin.x;
        let y_offset = HALF_TILE_HEIGHT + self.origin.y;
        Vec2::new(x_offset, y_offset)
    }
}

pub fn find_origin(windowsize: Vec2) -> Vec2 {
    let window_height = windowsize.y;
    let window_width = windowsize.x;
    let board_height = BOARD_HEIGHT as f32 * TILE_HEIGHT;
    let board_width = BOARD_WIDTH as f32 * TILE_WIDTH;

    let top_margin = (window_height / 2.0) - (board_height / 2.0);
    println!("Top Margin: {}", top_margin);

    let top_left_x = (window_width / 2.0) - (top_margin + board_width);
    let top_left_y = -(board_height / 2.0);

    println!("Board top_left: {}, {}", top_left_x, top_left_y);
    Vec2::new(top_left_x, top_left_y)
}
