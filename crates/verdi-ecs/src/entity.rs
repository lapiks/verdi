use crate::world::World;

pub type EntityId = u64;

pub struct EntityRef<'a> {
    world: &'a mut World,
    entity: EntityId,
}

impl<'a> EntityRef<'a> {
    pub(crate) fn new(world: &'a mut World, entity: EntityId) -> Self {
        Self { 
            world, 
            entity 
        }
    }

    pub fn add<ComponentType: 'static>(&mut self, component: ComponentType) -> &mut Self {
        self.world.add_component_to_entity(self.entity, component);
        self
    }

    pub fn remove<ComponentType: 'static>(&mut self) -> &mut Self {
        self.world.remove_component_from_entity::<ComponentType>(self.entity);
        self
    }

    pub fn id(&self) -> EntityId {
        self.entity
    }
}