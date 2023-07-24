use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub tiles: Handle<TextureAtlas>,
}

#[derive(Resource, Copy, Clone, Debug)]
pub struct DespawnTile;

#[derive(Resource, Copy, Clone, Debug)]

pub struct SelectedTile(pub UVec2);

impl SelectedTile {
    pub fn as_uvec2(&self) -> UVec2 {
        self.0
    }
}
