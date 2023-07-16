use crate::*;
use bevy::math::prelude::*;
use bevy::render::view::visibility;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::HashSet;

const SQUARE_COORD: [(i8, i8); 8] = [
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
];

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
        app.add_startup_system(create_gameboard.in_base_set(StartupSet::Startup))
            .add_startup_system(fill_gameboard.in_base_set(StartupSet::PostStartup));
        //.add_system(match_remove_refill);
    }
}

impl GameBoard {
    pub fn new(dimensions: UVec2, windowsize: Vec2) -> GameBoard {
        let origin = find_origin(windowsize);

        Self {
            dimensions,
            forward: vec![Some(tile::TileDesc::new()); (dimensions.x * dimensions.y) as usize],
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

    pub fn idx(&self, grid_pos: UVec2) -> usize {
        grid_pos.y as usize * self.dimensions.x as usize + grid_pos.x as usize
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

        #[cfg(feature = "debug")]
        println!("Looking for tile in {}, {}", grid_x, grid_y);

        let grid_pos = UVec2::new((grid_x / TILE_WIDTH) as u32, (grid_y / TILE_HEIGHT) as u32);

        #[cfg(feature = "debug")]
        println!("x: {}, y: {}", grid_pos.x, grid_pos.y);

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

    pub fn calculate_matches(&self) -> Vec<Entity> {
        todo!();
    }

    pub fn resolve_horizontal_matches(
        &mut self,
        tile_query: &Query<(Entity, &TileDesc, &TilePosition)>,
        to_be_deleted: &mut HashSet<Entity>,
    ) {
        for y in 0..self.dimensions.y {
            let mut match_counter: u32 = 1;
            let mut color_to_match = self.forward[self.idx((0, y).into())].unwrap().color;

            for x in 1..self.dimensions.x {
                let next_entity_color = self.forward[self.idx((x, y).into())].unwrap().color;
                if next_entity_color == color_to_match {
                    match_counter += 1;
                } else {
                    color_to_match = next_entity_color;

                    if match_counter >= MIN_MATCH_LENGTH {
                        let first_match = x - match_counter;
                        for backtrace in first_match..x {
                            let grid_pos = self.idx((backtrace, y).into());
                            self.forward[grid_pos] = None;
                            to_be_deleted.insert(
                                self.backward
                                    .remove(&self.idx((backtrace, y).into()))
                                    .unwrap(),
                            );
                            #[cfg(feature = "debug")]
                            println!("Pushed tile to be deleted at {}, {}", backtrace, y);
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
                    let grid_pos = self.idx((backtrace, y).into());
                    self.forward[grid_pos] = None;
                    to_be_deleted.insert(
                        self.backward
                            .remove(&self.idx((backtrace, y).into()))
                            .unwrap(),
                    );
                    #[cfg(feature = "debug")]
                    println!("Pushed tile to be deleted at {}, {}", backtrace, y);
                }
            }
        }
    }

    pub fn resolve_vertical_matches(
        &mut self,
        tile_query: &Query<(Entity, &TileDesc, &TilePosition)>,
        to_be_deleted: &mut HashSet<Entity>,
    ) {
        for x in 0..self.dimensions.x {
            let mut match_counter: u32 = 1;
            let mut color_to_match = self.forward[self.idx((x, 0).into())].unwrap().color;

            for y in 1..self.dimensions.y {
                let next_entity_color = self.forward[self.idx((x, y).into())].unwrap().color;
                if next_entity_color == color_to_match {
                    match_counter += 1;
                } else {
                    color_to_match = next_entity_color;

                    if match_counter >= MIN_MATCH_LENGTH {
                        let first_match = y - match_counter;
                        for backtrace in first_match..y {
                            let grid_pos = self.idx((x, backtrace).into());
                            self.forward[grid_pos] = None;
                            to_be_deleted.insert(
                                self.backward
                                    .remove(&self.idx((x, backtrace).into()))
                                    .unwrap(),
                            );
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
                    let grid_pos = self.idx((x, backtrace).into());
                    self.forward[grid_pos] = None;
                    to_be_deleted.insert(
                        self.backward
                            .remove(&self.idx((x, backtrace).into()))
                            .unwrap(),
                    );
                    #[cfg(feature = "debug")]
                    println!("Pushed tile to be deleted at {}, {}", x, backtrace);
                }
            }
        }
    }

    pub fn remove_matches(&mut self, mut commands: Commands, to_be_deleted: &HashSet<Entity>) {
        for entity in to_be_deleted {
            /* let tile_index = self.backward.remove(entity).unwrap();
            self.forward[tile_index] = None; */
            commands.entity(*entity).despawn();
        }
    }
}

pub fn find_origin(windowsize: Vec2) -> Vec2 {
    let window_height = windowsize.y;
    let window_width = windowsize.x;
    let board_height = BOARD_HEIGHT as f32 * TILE_HEIGHT;
    let board_width = BOARD_WIDTH as f32 * TILE_WIDTH;

    let top_margin = (window_height / 2.0) - (board_height / 2.0);
    #[cfg(feature = "debug")]
    println!("Top Margin: {}", top_margin);

    let top_left_x = (window_width / 2.0) - (top_margin + board_width);
    let top_left_y = -(board_height / 2.0);

    #[cfg(feature = "debug")]
    println!("Board top_left: {}, {}", top_left_x, top_left_y);
    Vec2::new(top_left_x, top_left_y)
}

pub fn create_gameboard(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let dimensions = UVec2::new(BOARD_HEIGHT, BOARD_WIDTH);
    let gameboard = board::GameBoard::new(dimensions, window_size);
    commands.insert_resource(gameboard);
}

pub fn fill_gameboard(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_board: ResMut<GameBoard>,
) {
    let offset = game_board.get_offsets();
    #[cfg(feature = "debug")]
    println!("x offset: {}", offset.x);
    #[cfg(feature = "debug")]
    println!("y offset: {}", offset.y);
    // let mut grid_pos = UVec2::new(0, 0);

    let board_entity = commands
        .spawn_empty()
        .insert(Name::new("Board"))
        .insert(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 1.)),
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
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
                    //let world_pos = game_board.get_world_pos(grid_pos);
                    let tile_desc = game_board.forward[index].unwrap();
                    //println!("Found empty tile, Generating {:?}", tile_desc);
                    let tile_entity = parent
                        .spawn(SpriteSheetBundle {
                            texture_atlas: game_assets.tiles.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    (x as f32 * tile_width) + (tile_width / 2.),
                                    (y as f32 * tile_width) + (tile_width / 2.),
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
    tile_query: Query<(Entity, &TileDesc, &TilePosition)>,
    mut game_board: ResMut<GameBoard>,
) {
    let mut to_be_deleted: HashSet<Entity> = HashSet::new();
    game_board.resolve_horizontal_matches(&tile_query, &mut to_be_deleted);
    game_board.resolve_vertical_matches(&tile_query, &mut to_be_deleted);
    game_board.remove_matches(commands, &to_be_deleted);
}
