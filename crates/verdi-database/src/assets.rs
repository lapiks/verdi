// use std::ops::Deref;

// use slotmap::SecondaryMap;

// use crate::{ResourceId, Resource};


// pub trait Asset {

// }

// pub struct Assets(SecondaryMap<ResourceId, Box<dyn Asset>>);

// impl Assets {
//     pub fn new() -> Self {
//         Self(SecondaryMap::default())
//     }
// }

// impl Resource for Assets {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

// impl Deref for Assets {
//     type Target = SecondaryMap<ResourceId, Box<dyn Asset>>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }    
// }