use std::collections::HashMap;

use glium::{texture::SrgbTexture2d as GpuTexture, Display};

use crate::{assets::AssetId, image::Image};

pub struct GpuAssets {
    textures: HashMap<AssetId, GpuTexture>,
}

impl GpuAssets {
    pub fn new() -> Self {
        Self { textures: HashMap::default(), }
    }

    pub fn add_texture(&mut self, display: &Display, id: AssetId, image: &Image) -> &GpuTexture {
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.get_data().as_raw(), image.get_dimensions());
        let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
        self.textures.insert(id, texture);

        self.textures.get(&id).unwrap()
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&GpuTexture> {
        self.textures.get(&id)
    }
}