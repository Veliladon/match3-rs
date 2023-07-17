mod board;
mod components;
mod distance;
mod effects;
mod pointer;
mod resources;
mod tile;
mod tilemove;

pub use crate::board::*;
pub use crate::components::*;
pub use crate::distance::CDistance;
pub use crate::distance::LDistance;
pub use crate::effects::*;
pub use crate::pointer::*;
pub use crate::resources::*;
pub use crate::tile::*;
pub use crate::tilemove::*;

pub use bevy::window::CursorGrabMode;
pub use bevy::{math::prelude::*, prelude::*, window::PrimaryWindow};

pub use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub use rand::prelude::*;

pub use std::ops::Deref;

const BACKGROUND: &str = "background.png";
const TILE_SHEET: &str = "match3.png";
const SHEET_TILE_WIDTH: f32 = 32.0;
const SHEET_TILE_HEIGHT: f32 = 32.0;
const BOARD_WIDTH: u32 = 8;
const BOARD_HEIGHT: u32 = 8;
const SPRITE_SCALE: f32 = 2.0;
const BORDER_SIZE: f32 = 5.0;
const TILE_WIDTH: f32 = SHEET_TILE_WIDTH * SPRITE_SCALE;
const TILE_HEIGHT: f32 = SHEET_TILE_HEIGHT * SPRITE_SCALE;
const HALF_TILE_WIDTH: f32 = TILE_WIDTH / 2.0;
const HALF_TILE_HEIGHT: f32 = TILE_HEIGHT / 2.0;
const MIN_MATCH_LENGTH: u32 = 3;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PointerPlugin)
        .add_plugin(EffectsPlugin)
        .add_plugin(TileMovePlugin)
        .add_plugin(GameBoardPlugin)
        .add_startup_system(setup_system.in_base_set(StartupSet::Startup))
        .add_startup_system(draw_background.in_base_set(StartupSet::PostStartup))
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let background_handle: Handle<Image> = asset_server.load(BACKGROUND);

    let tile_texture_handle = asset_server.load(TILE_SHEET);
    let tile_texture_atlas = TextureAtlas::from_grid(
        tile_texture_handle,
        Vec2::new(SHEET_TILE_WIDTH, SHEET_TILE_HEIGHT),
        12,
        9,
        None,
        None,
    );
    let tile_atlas = texture_atlases.add(tile_texture_atlas);
    let game_assets = GameAssets {
        background: background_handle,
        tiles: tile_atlas,
    };

    commands.insert_resource(game_assets);
}

fn draw_background(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut x_iterations = window.width() as u32 / 1024;
    let mut y_iterations = window.height() as u32 / 357;

    x_iterations += 1;
    y_iterations += 1;

    for y in 0..y_iterations {
        for x in 0..x_iterations {
            commands
                .spawn(SpriteBundle {
                    texture: game_assets.background.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * 1024.0 - 320.0,
                            y as f32 * 357.0 - 360.0,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Background);
        }
    }
}
