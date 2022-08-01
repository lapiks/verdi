use crate::entity::Entity;

pub struct World {
    entities_count: usize,
    entities: Vec<Entity>,
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            entities: Vec::new(),
        }
    }

    fn spawn(&mut self) -> usize {
        let entity_id = self.entities_count;

        self.entities_count += 1;
        entity_id
    }
}