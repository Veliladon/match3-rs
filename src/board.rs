use crate::*;
use bevy::math::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::HashSet;

/* const SQUARE_COORD: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
]; */

#[derive(Resource)]
pub struct GameBoard {
    pub dimensions: UVec2,
    pub forward: Vec<Option<TileDesc>>,
    pub backward: HashMap<usize, Entity>,
    pub origin: Vec2,
    pub entity: Entity,
}

pub struct GameBoardPlugin;

impl Plugin for GameBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_gameboard)
            .add_systems(PostStartup, fill_gameboard)
            .add_systems(
                Update,
                match_remove_refill.run_if(in_state(MoveState::NotMoving)),
            );
    }
}

pub trait Index2D<RHS = Self> {
    fn idx(&self, grid_pos: RHS) -> usize;
}

impl Index2D<(u32, u32)> for GameBoard {
    fn idx(&self, grid_pos: (u32, u32)) -> usize {
        grid_pos.1 as usize * self.dimensions.x as usize + grid_pos.0 as usize
    }
}

impl Index2D<UVec2> for GameBoard {
    fn idx(&self, grid_pos: UVec2) -> usize {
        grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize
    }
}

impl GameBoard {
    pub fn new(dimensions: UVec2, windowsize: Vec2) -> GameBoard {
        let origin = find_origin(windowsize);

        Self {
            dimensions,
            forward: vec![None; (dimensions.x * dimensions.y) as usize],
            backward: HashMap::new(),
            origin,
            entity: Entity::PLACEHOLDER,
        }
    }

    pub fn default() -> GameBoard {
        let origin = find_origin(Vec2::new(1280., 720.));

        Self {
            dimensions: (8, 8).into(),
            forward: vec![Some(tile::TileDesc::new()); (BOARD_HEIGHT * BOARD_WIDTH) as usize],
            backward: HashMap::new(),
            origin: origin,
            entity: Entity::PLACEHOLDER,
        }
    }

    pub fn find_local_from_grid(&self, position: UVec2) -> Vec2 {
        Vec2 {
            x: (position.x as f32 * TILE_WIDTH) + (TILE_WIDTH / 2.),
            y: (position.y as f32 * TILE_HEIGHT) + (TILE_HEIGHT / 2.),
        }
    }

    pub fn find_index_from_world(&self, position: Vec2) -> Option<usize> {
        let board_width = TILE_WIDTH * self.dimensions.x as f32;
        let board_height = TILE_HEIGHT * self.dimensions.y as f32;
        let grid_x = position.x - (TILE_WIDTH / 2.) - self.origin.x;
        let grid_y = position.y - (TILE_HEIGHT / 2.) - self.origin.y;

        if grid_x < 0.0 || grid_x > board_width {
            return None;
        }
        if grid_y < 0.0 || grid_y > board_height {
            return None;
        }
        let grid_pos = UVec2::new((grid_x / TILE_WIDTH) as u32, (grid_y / TILE_HEIGHT) as u32);
        Some(self.idx(grid_pos))
    }

    pub fn find_grid_from_index(&self, index: usize) -> UVec2 {
        UVec2::new(
            (index % self.dimensions.x as usize) as u32,
            (index / self.dimensions.y as usize) as u32,
        )
    }

    pub fn find_grid_from_world(&self, position: Vec2) -> Option<UVec2> {
        if let Some(index) = self.find_index_from_world(position) {
            Some(self.find_grid_from_index(index))
        } else {
            None
        }
    }

    pub fn get_tile(&self, grid_pos: UVec2) -> Option<TileDesc> {
        self.forward[grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize]
    }

    pub fn get_entity(&mut self, grid_pos: UVec2) -> Option<Entity> {
        self.backward
            .get(&(grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize))
            .copied()
    }

    pub fn get_board_pos(&self, grid_pos: UVec2) -> Vec2 {
        Vec2::new(
            (grid_pos.x as f32 * TILE_WIDTH) + (TILE_WIDTH / 2.),
            (grid_pos.y as f32 * TILE_WIDTH) + (TILE_WIDTH / 2.),
        )
    }

    pub fn get_world_pos(&self, grid_pos: UVec2) -> Vec2 {
        let offsets = self.get_offsets();
        Vec2::new(
            grid_pos.x as f32 * TILE_WIDTH + offsets.x,
            -(grid_pos.y as f32 * TILE_HEIGHT + offsets.y),
        )
    }

    pub fn get_offsets(&self) -> Vec2 {
        let x_offset = HALF_TILE_WIDTH + self.origin.x;
        let y_offset = HALF_TILE_HEIGHT + self.origin.y;
        Vec2::new(x_offset, y_offset)
    }

    pub fn resolve_horizontal_matches(&mut self, to_be_deleted: &mut HashSet<usize>) {
        for y in 0..self.dimensions.y {
            let mut match_counter: u32 = 1;
            let mut color_to_match = self.forward[self.idx((0, y))].unwrap().color;

            for x in 1..self.dimensions.x {
                let next_entity_color = self.forward[self.idx((x, y))].unwrap().color;
                if next_entity_color == color_to_match {
                    match_counter += 1;
                } else {
                    color_to_match = next_entity_color;

                    if match_counter >= MIN_MATCH_LENGTH {
                        let first_match = x - match_counter;
                        for backtrace in first_match..x {
                            let grid_index = self.idx((backtrace, y));
                            to_be_deleted.insert(grid_index);

                            info!("Pushed tile to be deleted at {}, {}", backtrace, y);
                        }
                    }
                    match_counter = 1;

                    if x >= self.dimensions.x - 2 {
                        break;
                    }
                }
            }
            if match_counter >= MIN_MATCH_LENGTH {
                let first_match = self.dimensions.x - match_counter;
                for backtrace in first_match..self.dimensions.x {
                    let grid_index = self.idx((backtrace, y));
                    to_be_deleted.insert(grid_index);

                    info!("Pushed tile to be deleted at {}, {}", backtrace, y);
                }
            }
        }
    }

    pub fn resolve_vertical_matches(&mut self, to_be_deleted: &mut HashSet<usize>) {
        for x in 0..self.dimensions.x {
            let mut match_counter: u32 = 1;
            let mut color_to_match = self.forward[self.idx((x, 0))].unwrap().color;

            for y in 1..self.dimensions.y {
                let next_entity_color = self.forward[self.idx((x, y))].unwrap().color;
                if next_entity_color == color_to_match {
                    match_counter += 1;
                } else {
                    color_to_match = next_entity_color;

                    if match_counter >= MIN_MATCH_LENGTH {
                        let first_match = y - match_counter;
                        for backtrace in first_match..y {
                            let grid_index = self.idx((x, backtrace));
                            to_be_deleted.insert(grid_index);

                            info!("Pushed tile to be deleted at {}, {}", x, backtrace);
                        }
                    }
                    match_counter = 1;

                    if y >= self.dimensions.y - 2 {
                        break;
                    }
                }
            }
            if match_counter >= MIN_MATCH_LENGTH {
                let first_match = self.dimensions.y - match_counter;
                for backtrace in first_match..self.dimensions.y {
                    let grid_index = self.idx((x, backtrace));
                    to_be_deleted.insert(grid_index);

                    info!("Pushed tile to be deleted at {}, {}", x, backtrace);
                }
            }
        }
    }

    pub fn remove_matches(&mut self, commands: &mut Commands, to_be_deleted: HashSet<usize>) {
        for index in to_be_deleted {
            self.forward[index] = None;
            let entity = self.backward.remove(&index).unwrap();

            commands.entity(entity).despawn();

            info!("Depawned: {:?}", entity);
        }
    }

    pub fn shuffle_tiles_down(&mut self, commands: &mut Commands) -> Vec<u32> {
        let mut column_spaces: Vec<u32> = Vec::new();
        let mut space_in_row;
        //let mut final_y = 0;
        for x in 0..BOARD_WIDTH {
            space_in_row = 0;
            for y in 0..BOARD_HEIGHT {
                let index = self.idx((x, y));

                if self.forward[index].is_none() {
                    for row in (y + 1)..BOARD_HEIGHT {
                        let row_index = self.idx((x, row));
                        if self.forward[row_index].is_some() {
                            self.forward[index] = self.forward[row_index];
                            self.forward[row_index] = None;
                            let new_entity = self.backward.remove(&row_index).unwrap();
                            self.backward.insert(index, new_entity);
                            let origin_transform = self.find_local_from_grid((x, row).into());
                            let destination_transform = self.find_local_from_grid((x, y).into());
                            commands.entity(new_entity).insert(TileMoving {
                                origin: origin_transform,
                                destination: destination_transform,
                                duration: Timer::from_seconds(0.5, TimerMode::Once),
                            });

                            info!("Moved tile from {}, {} to {}, {}", x, row, x, y);
                            //final_y = y;
                            //space_in_row = (BOARD_HEIGHT - 1) - final_y;
                            break;
                        }
                    }
                }
            }
            for y in 0..BOARD_HEIGHT {
                let index = self.idx((x, y));
                if self.forward[index].is_none() {
                    space_in_row += 1;
                }
            }
            column_spaces.push(space_in_row);

            if space_in_row > 0 {
                info!("Row {} found {} spaces", x, space_in_row);
            }
        }

        column_spaces
    }

    pub fn spawn_new_tiles(
        &mut self,
        commands: &mut Commands,
        column_spaces: Vec<u32>,
        game_assets: Res<GameAssets>,
    ) {
        for x in 0..BOARD_WIDTH as usize {
            let num_spaces = *column_spaces.get(x).unwrap();
            if num_spaces > 0 {
                for y in 1..=num_spaces {
                    let index = self.idx((x as u32, (BOARD_HEIGHT - y)));
                    let tile_desc = TileDesc::new();
                    self.forward[index] = Some(tile_desc);
                    let destination =
                        self.find_local_from_grid((x as u32, (BOARD_HEIGHT - y)).into());
                    let origin = Vec2::new(
                        destination.x,
                        destination.y + (BOARD_HEIGHT as f32 * TILE_HEIGHT),
                    );

                    commands.entity(self.entity).with_children(|parent| {
                        let tile_entity = parent
                            .spawn(SpriteSheetBundle {
                                atlas: TextureAtlas {
                                    layout: game_assets.tiles_layout.clone(),
                                    index: tile_desc.get_index(),
                                },
                                texture: game_assets.tiles.clone(),
                                transform: Transform {
                                    translation: Vec3::new(origin.x, origin.y, 2.0),
                                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                                    ..Default::default()
                                },
                                sprite: Sprite::default(),
                                ..default()
                            })
                            .insert(Tile)
                            .insert(tile_desc)
                            .insert(TilePosition((x as u32, BOARD_HEIGHT - y).into()))
                            .insert(TileMoving {
                                origin,
                                destination,
                                duration: Timer::from_seconds(0.5, TimerMode::Once),
                            })
                            .id();
                        self.backward.insert(index, tile_entity);

                        info!("Spawned a tile at: {}, {}", x, BOARD_HEIGHT - y);
                    });
                }
            }
        }
    }
}

pub fn find_origin(windowsize: Vec2) -> Vec2 {
    let window_height = windowsize.y;
    let window_width = windowsize.x;
    let board_height = BOARD_HEIGHT as f32 * TILE_HEIGHT;
    let board_width = BOARD_WIDTH as f32 * TILE_WIDTH;

    let top_margin = (window_height / 2.0) - (board_height / 2.0);

    let top_left_x = (window_width / 2.0) - (top_margin + board_width);
    let top_left_y = -(board_height / 2.0);

    Vec2::new(top_left_x, top_left_y)
}

pub fn create_gameboard(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let dimensions = UVec2::new(BOARD_HEIGHT, BOARD_WIDTH);
    let mut game_board = board::GameBoard::new(dimensions, window_size);

    for y in 0..game_board.dimensions.y {
        for x in 0..game_board.dimensions.x {
            let index = game_board.idx((x, y));
            let tile = TileDesc::new();
            game_board.forward[index] = Some(tile);
        }
    }

    check_intial_tiles(&mut game_board);

    commands.insert_resource(game_board);
    info!("Inserted Gameboard");
}

pub fn check_intial_tiles(game_board: &mut GameBoard) {
    let mut to_be_deleted: HashSet<usize> = HashSet::new();
    game_board.resolve_horizontal_matches(&mut to_be_deleted);
    game_board.resolve_vertical_matches(&mut to_be_deleted);
    if to_be_deleted.len() == 0 {
        return;
    }
    for index in to_be_deleted.iter() {
        game_board.forward[*index] = Some(TileDesc::new());
        info!("Replaced already matching tiles.")
    }
    check_intial_tiles(game_board);
}

pub fn fill_gameboard(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_board: ResMut<GameBoard>,
) {
    let offset = game_board.get_offsets();

    // let mut grid_pos = UVec2::new(0, 0);

    let board_entity = commands
        .spawn_empty()
        .insert(Name::new("Board"))
        .insert(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 1.)),
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(
                            TILE_WIDTH * BOARD_WIDTH as f32 + (2.0 * BORDER_SIZE),
                            TILE_HEIGHT * BOARD_HEIGHT as f32 + (2.0 * BORDER_SIZE),
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (BOARD_WIDTH / 2) as f32 * TILE_WIDTH,
                        (BOARD_HEIGHT / 2) as f32 * TILE_HEIGHT,
                        1.5,
                    )),
                    ..default()
                })
                .insert(BlackBackground);
        })
        .with_children(|parent| {
            let tile_width = TILE_WIDTH;
            let tile_height = TILE_HEIGHT;

            for y in 0..game_board.dimensions.y {
                for x in 0..game_board.dimensions.x {
                    let grid_pos = (x, y).into();
                    let index = game_board.idx(grid_pos);
                    let tile_desc = game_board.forward[index].unwrap();
                    let tile_entity = parent
                        .spawn(SpriteSheetBundle {
                            atlas: TextureAtlas {
                                layout: game_assets.tiles_layout.clone(),
                                index: tile_desc.get_index(),
                            },
                            texture: game_assets.tiles.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    (x as f32 * tile_width) + (tile_width / 2.),
                                    (y as f32 * tile_height) + (tile_height / 2.),
                                    2.0,
                                ),
                                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                                ..Default::default()
                            },
                            sprite: Sprite::default(),
                            ..Default::default()
                        })
                        .insert(Tile)
                        .insert(tile_desc)
                        .insert(TilePosition(grid_pos))
                        .id();
                    game_board.backward.insert(index, tile_entity);
                }
            }
        })
        .id();
    game_board.entity = board_entity;
}

pub fn match_remove_refill(
    mut commands: Commands,
    mut game_board: ResMut<GameBoard>,
    game_assets: Res<GameAssets>,
) {
    let mut to_be_deleted: HashSet<usize> = HashSet::new();
    game_board.resolve_horizontal_matches(&mut to_be_deleted);
    game_board.resolve_vertical_matches(&mut to_be_deleted);
    game_board.remove_matches(&mut commands, to_be_deleted);
    let column_spaces = game_board.shuffle_tiles_down(&mut commands);
    game_board.spawn_new_tiles(&mut commands, column_spaces, game_assets)
}
