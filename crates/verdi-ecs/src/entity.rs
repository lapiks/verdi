use std::{rc::Rc, cell::RefCell};

use mlua::{UserData, UserDataMethods};

use crate::world::World;

pub type EntityId = u64;

pub struct EntityRef {
    world: Rc<RefCell<World>>,
    entity: EntityId,
}

impl EntityRef {
    pub(crate) fn new(world: Rc<RefCell<World>>, entity: EntityId) -> Self {
        Self { 
            world, 
            entity 
        }
    }

    pub fn add<ComponentType: 'static>(&mut self, component: ComponentType) -> &mut Self {
        self.world.borrow_mut().add_component_to_entity(self.entity, component);
        self
    }

    pub fn remove<ComponentType: 'static>(&mut self) -> &mut Self {
        self.world.borrow_mut().remove_component_from_entity::<ComponentType>(self.entity);
        self
    }

    pub fn id(&self) -> EntityId {
        self.entity
    }
}

impl UserData for EntityRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // methods.add_method_mut("add", |_, mesh, ()| {
        //     Ok()
        // });

        // methods.add_method_mut("remove", |_, mesh, ()| {
        //     Ok()
        // });

        // methods.add_method("id", |_, entity, ()| {
        //     Ok(entity.id())
        // });
    }
}