use bevy::asset::AssetContainer;
use bevy::utils::HashMap;
use rand::prelude::*;

use crate::game::services::game_world::GameWorld;
use crate::game::vector::GridVector;

struct Dungeon {
    columns: HashMap<Point, bool>,
    horizontal_borders: HashMap<Point, BorderType>,
    vertical_borders: HashMap<Point, BorderType>,
    rooms: HashMap<Point, RoomType>,
}

impl Dungeon {
    fn new() -> Self {
        Self {
            columns: HashMap::new(),
            horizontal_borders: HashMap::new(),
            vertical_borders: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

pub struct DungeonSettings {
    pub anchor: GridVector,
    pub rooms_horizontal: usize,
    pub rooms_vertical: usize,
    pub rooms_width: usize,
    pub rooms_height: usize,
    pub spawn_wall_probability: f32,
    pub spawn_passege_probability: f32,
    pub spawn_door_probability: f32,
    pub fill_room_probability: f32,
    pub always_fill_outer_borders: bool,
    pub always_fill_lone_columns: bool,
    pub always_fill_closed_rooms: bool,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point(i32, i32);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum BorderType {
    Nothing,
    Wall,
    Passage,
    Door,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum RoomType {
    Empty,
    AttempedToFill,
}

pub fn create_dungeon(game_world: &mut GameWorld, settings: DungeonSettings, rng: &mut ThreadRng) {
    let dungeon = generate_dungeon(&settings, rng);
    add_to_world(game_world, dungeon, &settings);
}

fn generate_dungeon(settings: &DungeonSettings, rng: &mut ThreadRng) -> Dungeon {
    let mut dungeon = Dungeon::new();

    // horizontal borders
    for x in 0..settings.rooms_horizontal {
        for y in 0..=settings.rooms_vertical {
            let point = Point(x as i32, y as i32);

            if settings.always_fill_outer_borders && (y == 0 || y == settings.rooms_vertical) {
                dungeon.horizontal_borders.insert(point, BorderType::Wall);
            } else {
                dungeon
                    .horizontal_borders
                    .insert(point, select_border_type(rng, &settings));
            }
        }
    }

    // vertical borders
    for x in 0..=settings.rooms_horizontal {
        for y in 0..settings.rooms_vertical {
            let point = Point(x as i32, y as i32);

            if settings.always_fill_outer_borders && (x == 0 || x == settings.rooms_horizontal) {
                dungeon.vertical_borders.insert(point, BorderType::Wall);
            } else {
                dungeon
                    .vertical_borders
                    .insert(point, select_border_type(rng, &settings));
            }
        }
    }

    // columns
    for x in 0..=settings.rooms_horizontal {
        for y in 0..=settings.rooms_vertical {
            let point = Point(x as i32, y as i32);
            let point_left = Point(x as i32 - 1, y as i32);
            let point_down = Point(x as i32, y as i32 - 1);

            let need_column = || {
                !(test_horizontal_border(&dungeon, &point_left, &BorderType::Nothing)
                    && test_horizontal_border(&dungeon, &point, &BorderType::Nothing)
                    && test_vertical_border(&dungeon, &point_down, &BorderType::Nothing)
                    && test_vertical_border(&dungeon, &point, &BorderType::Nothing))
            };

            dungeon
                .columns
                .insert(point, settings.always_fill_lone_columns || need_column());
        }
    }

    // rooms
    for x in 0..settings.rooms_horizontal {
        for y in 0..settings.rooms_vertical {
            let point = Point(x as i32, y as i32);
            let point_right = Point(x as i32 + 1, y as i32);
            let point_up = Point(x as i32, y as i32 + 1);

            let closed_room = || {
                test_horizontal_border(&dungeon, &point, &BorderType::Wall)
                    && test_horizontal_border(&dungeon, &point_up, &BorderType::Wall)
                    && test_vertical_border(&dungeon, &point, &BorderType::Wall)
                    && test_vertical_border(&dungeon, &point_right, &BorderType::Wall)
            };

            if settings.always_fill_closed_rooms && closed_room() {
                dungeon.rooms.insert(point, RoomType::AttempedToFill);
            } else {
                dungeon
                    .rooms
                    .insert(point, select_room_type(rng, &settings));
            }
        }
    }

    dungeon
}

fn add_to_world(game_world: &mut GameWorld, dungeon: Dungeon, settings: &DungeonSettings) {
    let anchor = settings.anchor;

    for (Point(x, y), is_column) in &dungeon.columns {
        let corner = GridVector::new(
            (settings.rooms_width + 1) as i32 * x,
            (settings.rooms_height + 1) as i32 * y,
        ) + anchor;

        if *is_column {
            game_world.add_wall(corner);
        } else {
            game_world.add_floor(corner);
        }
    }

    for (Point(x, y), border_type) in &dungeon.horizontal_borders {
        let corner = GridVector::new(
            (settings.rooms_width + 1) as i32 * x + 1,
            (settings.rooms_height + 1) as i32 * y,
        ) + anchor;

        let vecs = (0..settings.rooms_width)
            .map(|i| GridVector::new(i as i32, 0) + corner)
            .collect::<Vec<GridVector>>();

        add_border(game_world, border_type, vecs);
    }

    for (Point(x, y), border_type) in &dungeon.vertical_borders {
        let corner = GridVector::new(
            (settings.rooms_width + 1) as i32 * x,
            (settings.rooms_height + 1) as i32 * y + 1,
        ) + anchor;

        let vecs = (0..settings.rooms_height)
            .map(|i| GridVector::new(0, i as i32) + corner)
            .collect::<Vec<GridVector>>();

        add_border(game_world, border_type, vecs);
    }

    for (Point(x, y), room_type) in &dungeon.rooms {
        let corner = GridVector::new(
            (settings.rooms_width + 1) as i32 * x + 1,
            (settings.rooms_height + 1) as i32 * y + 1,
        ) + anchor;

        enum PointType {
            CornerNE,
            CornerSE,
            CornerSW,
            CornerNW,
            EdgeN,
            EdgeE,
            EdgeS,
            EdgeW,
            Center,
        }

        let mut vecs = Vec::new();
        let center_x = settings.rooms_width / 2;
        let center_y = settings.rooms_height / 2;
        for dx in 0..settings.rooms_width {
            for dy in 0..settings.rooms_height {
                let point_type = if dx < center_x && dy < center_y {
                    PointType::CornerSW
                } else if dx > center_x && dy < center_y {
                    PointType::CornerSE
                } else if dx < center_x && dy > center_y {
                    PointType::CornerNW
                } else if dx > center_x && dy > center_y {
                    PointType::CornerNE
                } else if dx == center_x && dy < center_y {
                    PointType::EdgeS
                } else if dx == center_x && dy > center_y {
                    PointType::EdgeN
                } else if dx < center_x && dy == center_y {
                    PointType::EdgeW
                } else if dx > center_x && dy == center_y {
                    PointType::EdgeE
                } else {
                    PointType::Center
                };

                vecs.push((GridVector::new(dx as i32, dy as i32) + corner, point_type));
            }
        }

        match room_type {
            RoomType::Empty => {
                for (vec, _) in &vecs {
                    game_world.add_floor(vec.to_owned());
                }
            }
            RoomType::AttempedToFill => {
                let point = Point(*x, *y);
                let point_right = Point(*x + 1, *y);
                let point_up = Point(*x, *y + 1);

                let ne_corner = !test_horizontal_border(&dungeon, &point_up, &BorderType::Nothing)
                    && !test_vertical_border(&dungeon, &point_right, &BorderType::Nothing);

                let se_corner = !test_horizontal_border(&dungeon, &point, &BorderType::Nothing)
                    && !test_vertical_border(&dungeon, &point_right, &BorderType::Nothing);

                let sw_corner = !test_horizontal_border(&dungeon, &point, &BorderType::Nothing)
                    && !test_vertical_border(&dungeon, &point, &BorderType::Nothing);

                let nw_corner = !test_horizontal_border(&dungeon, &point_up, &BorderType::Nothing)
                    && !test_vertical_border(&dungeon, &point, &BorderType::Nothing);

                let n_border = ne_corner
                    && nw_corner
                    && test_horizontal_border(&dungeon, &point_up, &BorderType::Wall);
                let e_border = ne_corner
                    && se_corner
                    && test_vertical_border(&dungeon, &point_right, &BorderType::Wall);
                let s_border = se_corner
                    && sw_corner
                    && test_horizontal_border(&dungeon, &point, &BorderType::Wall);
                let w_border = nw_corner
                    && sw_corner
                    && test_vertical_border(&dungeon, &point, &BorderType::Wall);

                let center = n_border && e_border && s_border && w_border;

                for (vec, point_type) in &vecs {
                    let fill_point = match point_type {
                        PointType::CornerNE if ne_corner => true,
                        PointType::CornerSE if se_corner => true,
                        PointType::CornerSW if sw_corner => true,
                        PointType::CornerNW if nw_corner => true,
                        PointType::EdgeN if n_border => true,
                        PointType::EdgeE if e_border => true,
                        PointType::EdgeS if s_border => true,
                        PointType::EdgeW if w_border => true,
                        PointType::Center if center => true,
                        _ => false,
                    };

                    if fill_point {
                        game_world.add_wall(vec.to_owned());
                    } else {
                        game_world.add_floor(vec.to_owned());
                    }
                }
            }
        }
    }
}

fn test_horizontal_border(dungeon: &Dungeon, point: &Point, border_type: &BorderType) -> bool {
    dungeon
        .horizontal_borders
        .get(point)
        .unwrap_or(&BorderType::Nothing)
        == border_type
}
fn test_vertical_border(dungeon: &Dungeon, point: &Point, border_type: &BorderType) -> bool {
    dungeon
        .vertical_borders
        .get(point)
        .unwrap_or(&BorderType::Nothing)
        == border_type
}

fn select_border_type(rng: &mut ThreadRng, settings: &DungeonSettings) -> BorderType {
    let nothing_probability = (1.0
        - settings.spawn_wall_probability
        - settings.spawn_passege_probability
        - settings.spawn_door_probability)
        .min(0.0);

    let weights = |border_type: &BorderType| match border_type {
        BorderType::Nothing => nothing_probability,
        BorderType::Wall => settings.spawn_wall_probability,
        BorderType::Passage => settings.spawn_passege_probability,
        BorderType::Door => settings.spawn_door_probability,
    };

    [
        BorderType::Wall,
        BorderType::Door,
        BorderType::Passage,
        BorderType::Nothing,
    ]
    .choose_weighted(rng, weights)
    .unwrap()
    .to_owned()
}

fn select_room_type(rng: &mut ThreadRng, settings: &DungeonSettings) -> RoomType {
    let weights = |room_type: &RoomType| match room_type {
        RoomType::Empty => 1.0 - settings.fill_room_probability,
        RoomType::AttempedToFill => settings.fill_room_probability,
    };

    [RoomType::Empty, RoomType::AttempedToFill]
        .choose_weighted(rng, weights)
        .unwrap()
        .to_owned()
}

fn add_border(game_world: &mut GameWorld, border_type: &BorderType, vecs: Vec<GridVector>) {
    match border_type {
        BorderType::Nothing => {
            for vec in &vecs {
                game_world.add_floor(vec.to_owned());
            }
        }
        BorderType::Wall | BorderType::Passage | BorderType::Door => {
            for vec in &vecs {
                game_world.add_wall(vec.to_owned());
            }
        }
    }

    if border_type == &BorderType::Passage {
        game_world.add_floor((&vecs[&vecs.len() / 2]).to_owned());
    }

    if border_type == &BorderType::Door {
        game_world.add_door((&vecs[&vecs.len() / 2]).to_owned(), true);
    }
}
