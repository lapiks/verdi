use crate::component::{ComponentId, ComponentVec};

// Component list that defines the archetype
pub type Type = Vec<ComponentId>; 


pub struct Archetype {
    r#type: Type,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}