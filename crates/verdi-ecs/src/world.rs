use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::{EntityId, EntityHandle};
use crate::component::{ComponentVec};

#[derive(Clone)]
pub struct WorldHandle {
    inner: Rc<RefCell<World>>,
}

impl WorldHandle {
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        Self {
            inner: world,
        }
    }

    pub fn spawn(&self) -> EntityHandle {
        EntityHandle::new(
            self.inner.clone(), 
            self.inner.borrow_mut().spawn()
        )
    }

    pub fn despawn(&self, entity: EntityId) {
        // que retourner à lua ? EntityError ?
        self.inner.borrow_mut().despawn(entity);
    }

    pub fn entity(&self, entity: EntityId) -> Option<EntityHandle> {
        if let Some(entity_id) = self.inner.borrow_mut().entity(entity) {
            return Some(EntityHandle::new(
                self.inner.clone(), 
                entity_id
            ));
        }
        None
    }

    pub fn register_component<ComponentType: 'static>(&self) {
        self.inner.borrow_mut().register_component::<ComponentType>();
    }
}

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

    pub fn spawn(&mut self) -> EntityId {
        let entity_id = self.entities_count;
        // add a new empty entry in each component columns
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        self.entities.push(entity_id as EntityId);

        entity_id as EntityId
    }

    pub fn despawn(&mut self, entity: EntityId) -> EntityResult {
        match self.entity(entity) {
            None => Err(EntityError::UnknownEntity),
            _ => {
                for component_vec in self.component_vecs.iter_mut() {
                    component_vec.set_none(entity);
                }
                self.entities.remove(entity as usize);
                Ok(true)
            }
        }
    }

    pub fn entity(&mut self, entity: EntityId) -> Option<EntityId> {
        let id = self.entities.get(entity as usize)?;
        Some(*id)
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
        self.push_component_vec::<ComponentType>(new_component_vec);
    }

    pub(crate) fn remove_component_from_entity<ComponentType: 'static>(
        &mut self,
        entity: EntityId
    ) {
        // Find the good component vec
        if let Some(component_vec) = self.borrow_mut_component_vec::<ComponentType>() {
            component_vec.set_none(entity); // remove
            // check if any entity still uses this component, if not we can unregister it
            if component_vec.empty() == true {
                self.unregister_component::<ComponentType>();
            }
        }
    }

    fn push_component_vec<ComponentType: 'static>(&mut self, component_vec: Vec<Option<ComponentType>>) {
        self.component_vecs.push(
            Box::new(component_vec));
    }

    fn register_component<ComponentType: 'static>(&mut self) {
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        // The existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push_none();
        }

        // Register the new component type
        self.push_component_vec::<ComponentType>(new_component_vec);
    }

    fn unregister_component<ComponentType: 'static>(&mut self) {
        let mut n = 0;
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                break;
            }
            n += 1;
        }
        self.component_vecs.remove(n);
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

    fn borrow_mut_component_vec<ComponentType: 'static>(&mut self) -> Option<&mut Vec<Option<ComponentType>>> {
        // try to find a matching component vec
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }
}