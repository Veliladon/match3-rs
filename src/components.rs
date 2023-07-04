use bevy::prelude::Component;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Tile;

#[derive(Copy, Clone, PartialEq, Component)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize,
}
