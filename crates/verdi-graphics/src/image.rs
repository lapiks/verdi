  
use std::{path::Path, ops::Deref};
use image::{io::Reader as ImageReader, RgbaImage, ImageError};
use mlua::UserData;
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset, GpuAssets}, 
    gpu_image::GpuImage
};

pub type ImageId = ResourceId;

#[derive(Clone)]
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

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl PrepareAsset for Image {
    fn prepare_rendering(&self, ctx: &mut dyn miniquad::RenderingBackend, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        Ok(
            Box::new(
                GpuImage::new(ctx, self)
            )
        )
    }
}

#[derive(Clone)]
pub struct ImageHandle(Handle);

impl Deref for ImageHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImageHandle {
    pub fn new(assets: Assets, id: ImageId) -> Self {
        ImageHandle(assets.new_handle(id))
    }
}

impl UserData for ImageHandle {}