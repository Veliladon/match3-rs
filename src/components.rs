use bevy::prelude::Component;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct SelectedTile;

#[derive(Copy, Clone, PartialEq, Component)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize,
}
