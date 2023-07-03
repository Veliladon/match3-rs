mod board;
mod tile;
pub use crate::board::*;
pub use crate::tile::*;
pub use bevy::{prelude::*, window::PrimaryWindow};
pub use rand::prelude::*;

const BACKGROUND: &str = "background.png";
const TILE_SHEET: &str = "match3.png";
const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;
const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

#[derive(Resource)]
struct GameAssets {
    background: Handle<Image>,
    tiles: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup_system.in_base_set(StartupSet::Startup))
        .add_startup_system(create_gameboard.in_base_set(StartupSet::Startup))
        .add_startup_system(draw_initial_gamestate.in_base_set(StartupSet::PostStartup))
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let background_handle: Handle<Image> = asset_server.load(BACKGROUND);

    let tile_texture_handle = asset_server.load(TILE_SHEET);
    let tile_texture_atlas = TextureAtlas::from_grid(
        tile_texture_handle,
        Vec2::new(TILE_WIDTH, TILE_HEIGHT),
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

fn create_gameboard(mut commands: Commands) {
    let gameboard = board::GameBoard::new();
    commands.insert_resource(gameboard);
}

fn draw_initial_gamestate(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_board: Res<GameBoard>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
}
