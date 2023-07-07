use crate::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub tiles: Handle<TextureAtlas>,
}

#[derive(Resource)]

pub struct SelectedTile(UVec2);

pub impl SelectedTile {
    pub fn as_uvec2(&self) -> UVec2 {
        self.0
    }
}
