use crate::game::services::game_world::GameWorld;
use crate::game::vector::GridVector;
use bevy::utils::HashMap;
use rand::prelude::*;
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum EdgeType {
    NoWall,
    Wall,
    Passage,
    Door(bool),
}

struct Dungeon {
    columns: HashMap<Point, bool>,
    edges_horizontal: HashMap<Point, EdgeType>,
    edges_vertical: HashMap<Point, EdgeType>,
    rooms: HashMap<Point, bool>,
}

impl Dungeon {
    fn new() -> Self {
        Self {
            columns: HashMap::new(),
            edges_horizontal: HashMap::new(),
            edges_vertical: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

pub fn create_dungeon(
    game_world: &mut GameWorld,
    anchor: GridVector,
    rooms_horizontal: usize,
    rooms_vertical: usize,
    rng: &mut ThreadRng,
) {
    let dungeon = generate_dungeon(rooms_horizontal, rooms_vertical, rng);
    add_to_world(dungeon, game_world, anchor);
}

fn generate_dungeon(
    rooms_horizontal: usize,
    rooms_vertical: usize,
    rng: &mut ThreadRng,
) -> Dungeon {
    let mut dungeon = Dungeon::new();

    for x in 0..rooms_horizontal {
        for y in 0..=rooms_vertical {
            let point = Point::new(2 * x + 1, 2 * y);

            if y == 0 || y == rooms_vertical {
                dungeon.edges_horizontal.insert(point, EdgeType::Wall);
            } else {
                dungeon.edges_horizontal.insert(point, get_wall_type(rng));
            }
        }
    }

    for x in 0..=rooms_horizontal {
        for y in 0..rooms_vertical {
            let point = Point::new(2 * x, 2 * y + 1);

            if x == 0 || x == rooms_horizontal {
                dungeon.edges_vertical.insert(point, EdgeType::Wall);
            } else {
                dungeon.edges_vertical.insert(point, get_wall_type(rng));
            }
        }
    }

    for x in 0..=rooms_horizontal {
        for y in 0..=rooms_vertical {
            let point = Point::new(2 * x, 2 * y);

            if x == 0 || x == rooms_horizontal || y == 0 || y == rooms_vertical {
                dungeon.columns.insert(point, true);
            } else {
                let can_put_column = !(dungeon.edges_horizontal.get(&point.left())
                    == Some(&EdgeType::NoWall)
                    && dungeon.edges_horizontal.get(&point.right()) == Some(&EdgeType::NoWall)
                    && dungeon.edges_vertical.get(&point.up()) == Some(&EdgeType::NoWall)
                    && dungeon.edges_vertical.get(&point.down()) == Some(&EdgeType::NoWall));

                dungeon.columns.insert(point, can_put_column);
            }
        }
    }

    for x in 0..rooms_horizontal {
        for y in 0..rooms_vertical {
            let point = Point::new(2 * x + 1, 2 * y + 1);

            let check_edges = |edge_type: &EdgeType| {
                dungeon.edges_horizontal.get(&point.up()) == Some(edge_type)
                    && dungeon.edges_horizontal.get(&point.down()) == Some(edge_type)
                    && dungeon.edges_vertical.get(&point.right()) == Some(edge_type)
                    && dungeon.edges_vertical.get(&point.left()) == Some(edge_type)
            };

            let must_fill = check_edges(&EdgeType::Wall);

            if must_fill {
                dungeon.rooms.insert(point, true);
            } else {
                let can_fill = !check_edges(&EdgeType::NoWall);

                dungeon
                    .rooms
                    .insert(point, must_fill || (can_fill && rng.gen_bool(0.5)));
            }
        }
    }

    dungeon
}

fn add_to_world(dungeon: Dungeon, game_world: &mut GameWorld, anchor: GridVector) {
    for (point, is_column) in dungeon.columns {
        let vec = GridVector::new(2 * point.x + anchor.x, 2 * point.y + anchor.y);
        if is_column {
            game_world.add_wall(vec);
        } else {
            game_world.add_floor(vec);
        }
    }

    for (point, edge_horizontal) in &dungeon.edges_horizontal {
        let x = 2 * point.x - 1 + anchor.x;
        let y = 2 * point.y + anchor.y;

        let edge_vecs = (0..3)
            .map(|i| GridVector::new(x + i, y))
            .collect::<Vec<GridVector>>();

        create_edge(game_world, *edge_horizontal, edge_vecs);
    }

    for (point, edge_vertical) in &dungeon.edges_vertical {
        let x = 2 * point.x + anchor.x;
        let y = 2 * point.y - 1 + anchor.y;

        let edge_vecs = (0..3)
            .map(|i| GridVector::new(x, y + i))
            .collect::<Vec<GridVector>>();

        create_edge(game_world, *edge_vertical, edge_vecs);
    }

    for (point, filled) in dungeon.rooms {
        let x = 2 * point.x - 1 + anchor.x;
        let y = 2 * point.y - 1 + anchor.y;

        for dx in 0..3 {
            for dy in 0..3 {
                game_world.add_floor(GridVector::new(x + dx, y + dy));
            }
        }

        if filled {
            let ne_corner = dungeon.edges_horizontal.get(&point.up()) != Some(&EdgeType::NoWall)
                && dungeon.edges_vertical.get(&point.right()) != Some(&EdgeType::NoWall);
            let se_corner = dungeon.edges_horizontal.get(&point.down()) != Some(&EdgeType::NoWall)
                && dungeon.edges_vertical.get(&point.right()) != Some(&EdgeType::NoWall);
            let sw_corner = dungeon.edges_horizontal.get(&point.down()) != Some(&EdgeType::NoWall)
                && dungeon.edges_vertical.get(&point.left()) != Some(&EdgeType::NoWall);
            let nw_corner = dungeon.edges_horizontal.get(&point.up()) != Some(&EdgeType::NoWall)
                && dungeon.edges_vertical.get(&point.left()) != Some(&EdgeType::NoWall);

            let n_edge = ne_corner
                && nw_corner
                && dungeon.edges_horizontal.get(&point.up()) == Some(&EdgeType::Wall);
            let e_edge = ne_corner
                && se_corner
                && dungeon.edges_vertical.get(&point.right()) == Some(&EdgeType::Wall);
            let s_edge = se_corner
                && sw_corner
                && dungeon.edges_horizontal.get(&point.down()) == Some(&EdgeType::Wall);
            let w_edge = nw_corner
                && sw_corner
                && dungeon.edges_vertical.get(&point.left()) == Some(&EdgeType::Wall);

            let center = n_edge && e_edge && s_edge && w_edge;

            if ne_corner {
                game_world.add_wall(GridVector::new(x + 2, y + 2));
            }
            if se_corner {
                game_world.add_wall(GridVector::new(x + 2, y));
            }
            if nw_corner {
                game_world.add_wall(GridVector::new(x, y + 2));
            }
            if sw_corner {
                game_world.add_wall(GridVector::new(x, y));
            }
            if n_edge {
                game_world.add_wall(GridVector::new(x + 1, y + 2));
            }
            if e_edge {
                game_world.add_wall(GridVector::new(x + 2, y + 1));
            }
            if s_edge {
                game_world.add_wall(GridVector::new(x + 1, y));
            }
            if w_edge {
                game_world.add_wall(GridVector::new(x, y + 1));
            }
            if center {
                game_world.add_wall(GridVector::new(x + 1, y + 1));
            }
        }
    }
}

fn get_wall_type(rng: &mut ThreadRng) -> EdgeType {
    match rng.gen_range(0..4) {
        0 => EdgeType::Wall,
        1 => EdgeType::Door(true),
        2 => EdgeType::Passage,
        _ => EdgeType::NoWall,
    }
}

fn create_edge(game_world: &mut GameWorld, edge: EdgeType, edge_vecs: Vec<GridVector>) {
    match edge {
        EdgeType::NoWall => {
            game_world.add_floor(edge_vecs[0]);
            game_world.add_floor(edge_vecs[1]);
            game_world.add_floor(edge_vecs[2]);
        }
        EdgeType::Wall => {
            game_world.add_wall(edge_vecs[0]);
            game_world.add_wall(edge_vecs[1]);
            game_world.add_wall(edge_vecs[2]);
        }
        EdgeType::Passage => {
            game_world.add_wall(edge_vecs[0]);
            game_world.add_floor(edge_vecs[1]);
            game_world.add_wall(edge_vecs[2]);
        }
        EdgeType::Door(closed) => {
            game_world.add_wall(edge_vecs[0]);
            game_world.add_door(edge_vecs[1], closed);
            game_world.add_wall(edge_vecs[2]);
        }
    }
}
