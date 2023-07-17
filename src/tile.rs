use bevy::prelude::Component;

use num_derive::FromPrimitive;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]

pub enum TileColor {
    LightYellow = 0,
    LightPink = 1,
    DarkYellow = 2,
    BrightPink = 3,
    DarkGreen = 4,
    Red = 5,
    Green = 6,
    DarkRed = 7,
    LightGreen = 8,
    Brown = 9,
    LightBlue = 10,
    Orange = 11,
    DarkBlue = 12,
    LightGrey = 13,
    DarkPurple = 14,
    Grey = 15,
    BrightPurple = 16,
    DarkGrey = 17,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum TileMarking {
    Blank = 0,
    Cross = 1,
    Circle = 2,
    Square = 3,
    Triangle = 4,
    Star = 5,
}

#[derive(Copy, Clone, PartialEq, Debug, Component)]
pub struct TileDesc {
    pub color: TileColor,
    pub mark: TileMarking,
}

impl TileDesc {
    pub fn new() -> Self {
        let random_color: usize = thread_rng().gen_range(0..17);
        let random_mark: usize = thread_rng().gen_range(0..5);
        let new_color: TileColor = num::FromPrimitive::from_usize(random_color).unwrap();
        let new_mark: TileMarking = num::FromPrimitive::from_usize(random_mark).unwrap();

        Self {
            color: new_color,
            mark: new_mark,
        }
    }

    pub fn get_index(&self) -> usize {
        (self.color as usize * 6) + self.mark as usize
    }
}
