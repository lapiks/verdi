use crate::entity::EntityId;


pub trait ComponentVec {
    // A reference to a type implementing this trait should be able to convert to Any  
    fn as_any(&self) -> &dyn std::any::Any;
    // A mut reference to a type implementing this trait should be able to convert to mut Any  
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    // Push a None value (aka an absent component) into the ComponentVec
    fn push_none(&mut self);
    // Set a none value at Entity index
    fn set_none(&mut self, entity: EntityId);
    // Check if any entity still uses this component
    fn empty(&self) -> bool;
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }

    fn set_none(&mut self, entity: EntityId) {
        self[entity as usize] = None;
    }

    fn empty(&self) -> bool {
        for component in self.iter() {
            if component.is_some() {
                return false;
            }
        }
        true
    }
}
