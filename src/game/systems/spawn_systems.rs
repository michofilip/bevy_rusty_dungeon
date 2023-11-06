use bevy::prelude::*;

use crate::game::services;
use crate::game::vector::GridVector;

pub fn spawn_level(world: &mut World) {
    for y in -10..=10 {
        for x in -10..=10 {
            let vec = GridVector::new(x, y);
            services::spawners::spawn_floor(vec, world);

            if x == -10 || y == -10 || x == 10 || y == 10 {
                services::spawners::spawn_wall(vec, world);
            }
        }
    }

    services::spawners::spawn_player(GridVector::new(0, 0), 0.0, world);
    services::spawners::spawn_monster(GridVector::new(5, 5), 0.0, world);
    services::spawners::spawn_monster(GridVector::new(5, -5), 0.0, world);
    services::spawners::spawn_monster(GridVector::new(-5, 5), 0.0, world);
    services::spawners::spawn_monster(GridVector::new(-5, -5), 0.0, world);
}
