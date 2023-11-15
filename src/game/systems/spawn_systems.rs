use crate::game::services;
use bevy::prelude::*;

use crate::game::services::game_world::GameWorld;
use crate::game::vector::GridVector;

pub fn spawn_level(world: &mut World) {
    let mut game_world = GameWorld::empty();

    for y in -10..=10 {
        for x in -10..=10 {
            let vec = GridVector::new(x, y);
            game_world.add_floor(vec);

            if x == -10 || y == -10 || x == 10 || y == 10 {
                game_world.add_wall(vec);
            }
        }
    }

    game_world.add_player(GridVector::new(0, 0));
    game_world.add_monster(GridVector::new(5, 5));
    game_world.add_monster(GridVector::new(5, -5));
    game_world.add_monster(GridVector::new(-5, 5));
    game_world.add_monster(GridVector::new(-5, -5));

    game_world.spawn_world(world);
}

pub fn spawn_level_2(world: &mut World) {
    info!("create_level");

    let mut rng = rand::thread_rng();

    let rooms_horizontal = 9;
    let rooms_vertical = 9;

    let mut game_world = GameWorld::empty();

    let settings = services::dungeon_generator::DungeonSettings {
        anchor: GridVector::zero(),
        rooms_horizontal: rooms_horizontal,
        rooms_vertical: rooms_vertical,
        rooms_width: 3,
        rooms_height: 3,
        spawn_wall_probability: 0.25,
        spawn_passege_probability: 0.25,
        spawn_door_probability: 0.25,
        fill_room_probability: 0.5,
        always_fill_outer_borders: true,
        always_fill_lone_columns: false,
        always_fill_closed_rooms: true,
    };

    services::dungeon_generator::create_dungeon(&mut game_world, settings, &mut rng);

    game_world.add_player(GridVector::new(
        4 * (rooms_horizontal as i32 / 2) + 2,
        4 * (rooms_vertical as i32 / 2) + 2,
    ));

    game_world.add_monster(GridVector::new(4 * 0 + 2, 4 * 0 + 2));

    game_world.spawn_world(world);
}
