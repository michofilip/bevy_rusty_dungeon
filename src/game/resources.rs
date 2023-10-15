use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::game::vector::GridVector;

use super::action::Action;

#[derive(Resource, Default)]
pub struct Level {
    entities: HashMap<GridVector, HashSet<Entity>>,
}

impl Level {
    pub fn insert(&mut self, vec: GridVector, entity: Entity) {
        if let Some(entity_set) = self.entities.get_mut(&vec) {
            entity_set.insert(entity);
        } else {
            let mut entity_set = HashSet::new();
            entity_set.insert(entity);
            self.entities.insert(vec, entity_set);
        }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SelectedEntity(pub Option<Entity>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActionsToExecute(pub Vec<Action>);
