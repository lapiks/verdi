  
use std::{path::Path, cell::RefCell, rc::Rc};
use glium::Display;
use image::{io::Reader as ImageReader, RgbaImage, ImageError};
use mlua::UserData;
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets};

use crate::{
    gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset}, 
    gpu_image::GpuImage
};

pub type ImageId = ResourceId;

pub struct Image {
    width: u32,
    height: u32,
    data: RgbaImage,
    pub id: ImageId,
}

impl Resource for Image {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let dyn_img = ImageReader::open(path)?.decode()?;
        let rgba8_img = dyn_img.to_rgba8();

        let dim = rgba8_img.dimensions();

        Ok(Self { 
            width: dim.0, 
            height: dim.1,
            data: rgba8_img,
            id: ImageId::null(),
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
            id: ImageId::null(),
        })
    }

    pub fn get_data(&self) -> &RgbaImage {
        &self.data
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height)
    }
}

impl PrepareAsset for Image {
    fn prepare_rendering(&self, display: &Display, assets: &Assets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        Ok(
            Box::new(
                GpuImage::new(display, self)
            )
        )
    }
}

#[derive(Clone)]
pub struct ImageHandle {
    pub assets: Rc<RefCell<Assets>>,
    pub id: ImageId,
}

impl ImageHandle {
    pub fn new(assets: Rc<RefCell<Assets>>, id: ImageId) -> Self{
        Self { assets, id }
    }
}

impl UserData for ImageHandle {}