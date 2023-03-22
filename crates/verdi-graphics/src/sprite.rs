use std::{rc::Rc, cell::RefCell};

use mlua::UserData;
use slotmap::{new_key_type, Key};

use crate::{
    database::Database, 
    mesh::MeshId, 
    image::ImageId
};

new_key_type! {
    pub struct SpriteId;
}

pub struct Sprite {
    pub image_id: ImageId,
    pub quad_id: MeshId,
    pub id: SpriteId,
}

impl Sprite {
    pub fn new(image_id: ImageId, quad_id: MeshId) -> Self {
        Self {
            image_id,
            quad_id,
            id: SpriteId::null(),
        }
    }
}

#[derive(Clone)]
pub struct SpriteHandle {
    pub db: Rc<RefCell<Database>>,
    pub id: SpriteId,
}

impl UserData for SpriteHandle {}