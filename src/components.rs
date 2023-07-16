use crate::*;
use bevy::{
    hierarchy::ReportHierarchyIssue,
    prelude::Component,
    time::{Timer, TimerMode},
};
use std::ops::{Add, Deref, Sub};

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

impl Deref for TilePosition {
    type Target = UVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for TilePosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            0: (self.0.x + rhs.0.x, self.0.y + rhs.0.y).into(),
        }
    }
}

impl Sub for TilePosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            0: (
                self.0.x.saturating_sub(rhs.0.x),
                self.0.y.saturating_sub(rhs.0.y),
            )
                .into(),
        }
    }
}

impl Add<(i8, i8)> for TilePosition {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.0.x as i32) + x as i32) as u32;
        let y = ((self.0.y as i32) + y as i32) as u32;
        Self { 0: (x, y).into() }
    }
}
