use crate::game::services::spawners;
use crate::game::vector::GridVector;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub struct GameWorld {
    pub floors: HashSet<GridVector>,
    pub walls: HashSet<GridVector>,
    pub doors: HashMap<GridVector, bool>,
    pub player: Option<GridVector>,
    pub monsters: HashSet<GridVector>,
}

impl GameWorld {
    pub fn empty() -> Self {
        Self {
            floors: HashSet::new(),
            walls: HashSet::new(),
            doors: HashMap::new(),
            player: None,
            monsters: HashSet::new(),
        }
    }

    pub fn add_floor(&mut self, vec: GridVector) {
        self.floors.replace(vec);
        self.walls.remove(&vec);
        self.doors.remove(&vec);
        match self.player {
            Some(player_vec) if player_vec == vec => {
                self.player = None;
            }
            _ => {}
        }
        self.monsters.remove(&vec);
    }

    pub fn add_wall(&mut self, vec: GridVector) {
        self.add_floor(vec);
        self.walls.replace(vec);
    }

    pub fn add_door(&mut self, vec: GridVector, close: bool) {
        self.add_floor(vec);
        self.doors.insert(vec, close);
    }

    pub fn add_player(&mut self, vec: GridVector) {
        self.add_floor(vec);
        self.player = Some(vec);
    }

    pub fn add_monster(&mut self, vec: GridVector) {
        self.add_floor(vec);
        self.monsters.replace(vec);
    }

    pub fn spawn_world(&self, world: &mut World) {
        for coordinates in &self.floors {
            spawners::spawn_floor(coordinates.to_owned(), world);
        }

        for coordinates in &self.walls {
            spawners::spawn_wall(coordinates.to_owned(), world);
        }

        for (coordinates, closed) in &self.doors {
            spawners::spawn_door(coordinates.to_owned(), *closed, world);
        }

        if let Some(coordinates) = &self.player {
            spawners::spawn_player(coordinates.to_owned(), 0.0, world);
        }

        for coordinates in &self.monsters {
            spawners::spawn_monster(coordinates.to_owned(), 0.1, world);
        }
    }
}
