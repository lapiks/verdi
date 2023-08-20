mod assets;

use std::{any::Any, rc::Rc, cell::{RefCell, Ref, RefMut}, marker::PhantomData};

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct ResourceId;
}

pub trait Resource: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Assets(SlotMap<ResourceId, Box<dyn Resource>>);

impl Assets {
    pub fn new() -> Self {
        Self(SlotMap::default())
    }

    pub fn add(&mut self, res: Box<dyn Resource>) -> ResourceId {
        self.0.insert(res)
    }

    pub fn get<R: Any>(&self, id: ResourceId) -> Option<&R> {
        match self.0.get(id) {
            Some(value) => {
                return value.as_any().downcast_ref();
            },
            None => return None,
        };
    }

    pub fn get_mut<R: Any>(&mut self, id: ResourceId) -> Option<&mut R> {
        match self.0.get_mut(id) {
            Some(value) => {
                return value.as_any_mut().downcast_mut();
            },
            None => return None,
        };
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Resource for Assets {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Handle<R: Resource> {
    assets: Rc<RefCell<Assets>>,
    id: ResourceId,
    marker: PhantomData<fn() -> R>,
}

impl<R: Resource> Handle<R> {
    pub fn new(assets: Rc<RefCell<Assets>>, id: ResourceId) -> Self {
        Self { assets, id, marker: PhantomData }
    }

    pub fn get_id(&self) -> ResourceId {
        self.id
    }

    pub fn get_assets(&self) -> Ref<'_, Assets> {
        self.assets.borrow()
    }

    pub fn get_assets_mut(&mut self) -> RefMut<'_, Assets> {
        self.assets.borrow_mut()
    }
}

pub struct Database {
    images: Assets<Image>,
}

// impl Database {
//     pub fn add(&mut self, res: Box<dyn Resource>) -> ResourceId {
//         self.resources.insert(res)
//     }

//     pub fn get<R: Any>(&self, id: ResourceId) -> Option<&R> {
//         match self.resources.get(id) {
//             Some(value) => {
//                 return value.as_any().downcast_ref();
//             },
//             None => return None,
//         };
//     }

//     pub fn get_mut<R: Any>(&mut self, id: ResourceId) -> Option<&mut R> {
//         match self.resources.get_mut(id) {
//             Some(value) => {
//                 return value.as_any_mut().downcast_mut();
//             },
//             None => return None,
//         };
//     }

//     pub fn clear(&mut self) {
//         self.resources.clear();
//     }
// }

