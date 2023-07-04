mod board;
mod components;
mod tile;
pub use crate::board::*;
pub use crate::components::*;
pub use crate::tile::*;
pub use bevy::{prelude::*, window::PrimaryWindow};
pub use rand::prelude::*;

const BACKGROUND: &str = "background.png";
const TILE_SHEET: &str = "match3.png";
const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;
const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
const SPRITE_SCALE: f32 = 2.0;
const BORDER_SIZE: f32 = 5.0;

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
        .add_startup_system(draw_background.in_base_set(StartupSet::PostStartup))
        .add_startup_system(fill_gameboard.in_base_set(StartupSet::PostStartup))
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

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(
                    TILE_WIDTH * SPRITE_SCALE * BOARD_WIDTH as f32 + (2.0 * BORDER_SIZE),
                    TILE_HEIGHT * SPRITE_SCALE * BOARD_HEIGHT as f32 + (2.0 * BORDER_SIZE),
                )),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (BOARD_WIDTH / 2) as f32 * SPRITE_SCALE * TILE_WIDTH + 24.0,
                0.0,
                1.5,
            )),
            ..default()
        })
        .insert(Background);
}

fn fill_gameboard(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_board: ResMut<GameBoard>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    println!("Starting to assemble gameboard");
    println!("{:?}", game_board.forward);

    for y in 0..game_board.height {
        for x in 0..game_board.width {
            let index = game_board.idx(x, y);
            if game_board.forward.get(index).unwrap().is_none() {
                let tile_desc = tile::TileDesc::new();
                //println!("Found empty tile, Generating {:?}", tile_desc);
                let tile_entity = commands
                    .spawn(SpriteSheetBundle {
                        texture_atlas: game_assets.tiles.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x as f32 * SPRITE_SCALE * 32.0 + 56.0,
                                y as f32 * SPRITE_SCALE * 32.0
                                    - (BOARD_HEIGHT as f32 / 2.0 * TILE_HEIGHT * SPRITE_SCALE
                                        - (TILE_HEIGHT * SPRITE_SCALE / 2.0)),
                                2.0,
                            ),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(tile_desc.get_index()),
                        ..Default::default()
                    })
                    .insert(Tile)
                    .insert(tile_desc)
                    .insert(TilePosition { x: x, y: y })
                    .id();
                game_board.backward.insert(tile_entity, index);
                game_board.forward[index] = Some(tile_entity);
            }
        }
    }
}
