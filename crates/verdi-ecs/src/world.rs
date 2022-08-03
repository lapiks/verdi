use std::collections::HashMap;

use crate::archetype::{Archetype};
use crate::entity::{EntityId};
use crate::component::{ComponentId};

pub struct World {
    entities_count: usize,
    entity_index: HashMap<EntityId, Archetype>,
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            entity_index: HashMap::new(),
        }
    }

    fn spawn(&mut self) -> usize {
        let entity_id = self.entities_count;

        self.entities_count += 1;
        entity_id
    }
    
    fn has_component(&self, entity: EntityId, component: ComponentId) -> bool {
        let archetype = self.entity_index.get(&entity);
        match archetype {
            None => return false,
            Some(archetype) => 
                for c in archetype.iter() {
                    if *c == component {
                        return true;
                    }
                    
                }
        }
        false
    }
}