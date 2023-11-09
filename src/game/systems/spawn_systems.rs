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

    services::dungeon_generator::create_dungeon(
        &mut game_world,
        GridVector::zero(),
        rooms_horizontal,
        rooms_vertical,
        &mut rng,
    );

    game_world.add_player(GridVector::new(
        4 * (rooms_horizontal as i32 / 2) + 2,
        4 * (rooms_vertical as i32 / 2) + 2,
    ));

    game_world.add_monster(GridVector::new(4 * 0 + 2, 4 * 0 + 2));

    game_world.spawn_world(world);
}
