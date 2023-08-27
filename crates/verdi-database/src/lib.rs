use std::{any::Any, rc::Rc, cell::{RefCell, Ref, RefMut}};

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct ResourceId;
}

pub trait Resource: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone)]
pub struct Assets(Rc<RefCell<AssetDatas>>);

impl Assets {
    pub fn new_handle(&self, id: ResourceId) -> Handle {
        Handle::new(self.clone(), id)
    }
}

impl Assets {
    pub fn new() -> Self {
        Self(
            Rc::new(
                RefCell::new(
                    AssetDatas(
                        SlotMap::default()
                    )
                )
            )
        )
    }

    pub fn add(&mut self, res: Box<dyn Resource>) -> ResourceId {
        self.0.borrow_mut().0.insert(res)
    }

    pub fn clear(&mut self) {
        self.0.borrow_mut().0.clear();
    }

    pub fn get_datas(&self) -> Ref<'_, AssetDatas> {
        self.0.borrow()
    }

    pub fn get_datas_mut(&mut self) -> RefMut<'_, AssetDatas> {
        self.0.borrow_mut()
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

pub struct AssetDatas(SlotMap<ResourceId, Box<dyn Resource>>);

impl AssetDatas {
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
}

#[derive(Clone)]
pub struct Handle {
    assets: Assets,
    id: ResourceId,
}

impl Handle {
    pub fn new(assets: Assets, id: ResourceId) -> Self {
        Self { assets, id }
    }

    pub fn get_id(&self) -> ResourceId {
        self.id
    }

    pub fn get_assets(&self) -> &Assets {
        &self.assets
    }

    pub fn get_assets_mut(&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub fn get_datas(&self) -> Ref<'_, AssetDatas> {
        self.assets.get_datas()
    }

    pub fn get_datas_mut(&mut self) -> RefMut<'_, AssetDatas> {
        self.assets.get_datas_mut()
    }
}