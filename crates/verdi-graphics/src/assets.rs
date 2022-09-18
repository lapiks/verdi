use std::collections::HashMap;
use glium::Display;
use uuid::Uuid;
use crate::image::{Image, ImageRef};

pub type AssetId = Uuid;

#[derive(PartialEq)]
pub enum AssetState {
    Created,
    Loaded,
}

pub struct Assets {
    textures: HashMap<AssetId, Image>,
}

impl Assets {
    pub fn new() -> Self {
        Self { 
            textures: HashMap::default(),
        }
    }

    pub fn add_texture(&mut self, image: Image) -> ImageRef {
        let tex_id = Uuid::new_v4();
        self.textures.insert(tex_id, image);

        ImageRef::new(tex_id)
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&Image> {
        self.textures.get(&id)
    }
}