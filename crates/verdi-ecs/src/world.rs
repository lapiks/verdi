use crate::entity::{EntityId, EntityRef, self};
use crate::component::{ComponentVec};

pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>, // component storage as columns
    entities: Vec<EntityId>,
}

pub enum EntityError {
    UnknownEntity,
}

pub type EntityResult = Result<bool, EntityError>;

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityRef {
        let entity_id = self.entities_count;
        // add a new empty entry in each component columns
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        self.entities.push(entity_id as EntityId);
        EntityRef::new(self, entity_id as EntityId)
    }

    pub fn despawn(&mut self, entity: EntityId) -> EntityResult {
        match self.entity(entity) {
            None => Err(EntityError::UnknownEntity),
            Some(entity_ref) => {
                for component_vec in self.component_vecs.iter_mut() {
                    component_vec.set_none(entity);
                }
                self.entities.remove(entity as usize);
                Ok(true)
            }
        }
    }

    pub fn entity(&mut self, entity: EntityId) -> Option<EntityRef> {
        let id = self.entities.get(entity as usize)?;
        Some(EntityRef::new(self, *id))
    }

    pub(crate) fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: EntityId,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity as usize] = Some(component);
                return;
            }
        }
        // No matching component storage exists yet, so we have to register a new one.
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push_none();
        }

        // Give this Entity the Component.
        new_component_vec[entity as usize] = Some(component);

        // Register the new component type
        self.register_component::<ComponentType>(new_component_vec);
    }

    fn register_component<ComponentType: 'static>(&mut self, component_vec: Vec<Option<ComponentType>>) {
        self.component_vecs.push(
            Box::new(component_vec));
    }

    fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<&Vec<Option<ComponentType>>> {
        // try to find a matching component vec
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }
}