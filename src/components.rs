use crate::*;
use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct BlackBackground;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct TileMoving {
    pub origin: UVec2,
    pub destination: UVec2,
    pub duration: Timer,
}

#[derive(Component)]
pub struct TileHighlight(pub Timer);

impl Default for TileHighlight {
    fn default() -> Self {
        Self(Timer::from_seconds(0.8, TimerMode::Repeating))
    }
}

#[derive(Copy, Clone, PartialEq, Component, Reflect)]

pub struct TilePosition(pub UVec2);
