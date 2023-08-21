use std::ops::Deref;

use mlua::UserData;
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    mesh::MeshId, 
    image::ImageId
};

pub type SpriteId = ResourceId;

#[derive(Clone)]
pub struct Sprite {
    pub image_id: ImageId,
    pub quad_id: MeshId,
    pub id: SpriteId,
}

impl Resource for Sprite {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
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
pub struct SpriteHandle(Handle<Sprite>);

impl Deref for SpriteHandle {
    type Target = Handle<Sprite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SpriteHandle {
    pub fn new(assets: Assets, id: SpriteId) -> Self {
        SpriteHandle(assets.new_handle(id))
    }
}

impl UserData for SpriteHandle {}