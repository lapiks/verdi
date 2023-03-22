use std::{rc::Rc, cell::RefCell};

use mlua::UserData;
use slotmap::new_key_type;

use crate::database::Database;

new_key_type! {
    pub struct SpriteId;
}


pub struct Sprite {
    pub id: SpriteId,
}

#[derive(Clone)]
pub struct SpriteHandle {
    pub graph: Rc<RefCell<Database>>,
    pub id: SpriteId,
}

impl UserData for SpriteHandle {}