
use glium::Display;
use image::{io::Reader as ImageReader, RgbaImage, ImageError};
use rlua::UserData;
use uuid::Uuid;

use crate::{
    assets::{AssetId, AssetState}, 
    gpu_assets::GpuAssets, 
    gpu_image::GpuImage
};

pub struct Image {
    width: u32,
    height: u32,
    data: RgbaImage,
    state: AssetState,
    pub id: AssetId,
}

impl Image {
    pub fn new(path: &String) -> Result<Self, ImageError> {
        let dyn_img = ImageReader::open(path)?.decode()?;
        let rgba8_img = dyn_img.to_rgba8();

        let dim = rgba8_img.dimensions();

        Ok(Self { 
            width: dim.0, 
            height: dim.1,
            data: rgba8_img,
            state: AssetState::Created,
            id: Uuid::nil(),
        })
    }

    pub fn from_buffer(buffer: &[u8]) -> Result<Self, ImageError> {
        let mut reader = image::io::Reader::new(std::io::Cursor::new(buffer));
        reader.set_format(::image::ImageFormat::Png);
        reader.no_limits();

        let dyn_img = reader.decode()?;
        let rgba8_img = dyn_img.to_rgba8();

        let dim = rgba8_img.dimensions();
        
        Ok(Self { 
            width: dim.0, 
            height: dim.1,
            data: rgba8_img,
            state: AssetState::Created,
            id: Uuid::nil(),
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.state == AssetState::Loaded
    }

    pub fn set_loaded(&mut self) {
        self.state = AssetState::Loaded
    }

    pub fn get_data(&self) -> &RgbaImage {
        &self.data
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height)
    }

    pub fn prepare_rendering(&self, display: &Display, gpu_assets: &mut GpuAssets) {
        if gpu_assets.get_texture(self.id).is_none() {
            let gpu_image = GpuImage::new(display, self);
            gpu_assets.add_texture(self.id, gpu_image);
        }
    }
}

#[derive(Clone, Copy)]
pub struct ImageRef {
    pub id: AssetId,
}

impl ImageRef {
    pub fn new(id: AssetId) -> Self{
        Self { id }
    }
}

impl UserData for ImageRef {}