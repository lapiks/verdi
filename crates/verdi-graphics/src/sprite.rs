use std::{rc::Rc, cell::RefCell};

use mlua::UserData;
use slotmap::{new_key_type, Key};

use crate::{database::Database, mesh::MeshId};

new_key_type! {
    pub struct SpriteId;
}

pub struct Sprite {
    pub quad_id: MeshId,
    pub id: SpriteId,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            quad_id: MeshId::null(),
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