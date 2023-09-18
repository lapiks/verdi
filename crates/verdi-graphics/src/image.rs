  
use std::{path::Path, ops::Deref};
use glium::{Display, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter}};
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
    data: Option<RgbaImage>,
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
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            width: width, 
            height: height,
            data: None,
            id: ImageId::null(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let dyn_img = ImageReader::open(path)?.decode()?;
        let rgba8_img = dyn_img.to_rgba8();

        let dim = rgba8_img.dimensions();

        Ok(Self { 
            width: dim.0, 
            height: dim.1,
            data: Some(rgba8_img),
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
            data: Some(rgba8_img),
            id: ImageId::null(),
        })
    }

    pub fn get_data(&self) -> &Option<RgbaImage> {
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
    fn prepare_rendering(&self, ctx: &Display, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        if let Some(datas) = &self.get_data() {
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &datas.as_raw(), 
                self.get_dimensions()
            );

            let gl = glium::texture::SrgbTexture2d::new(
                ctx, 
                raw_image
            ).unwrap();

            // TODO: to be settable
            let sampler = SamplerBehavior {
                minify_filter: MinifySamplerFilter::Nearest,
                magnify_filter: MagnifySamplerFilter::Nearest,
                .. Default::default()
            };

            return Ok(
                Box::new(
                    GpuImage::new(gl, sampler)
                )
            )
        }
        else {
            let gl = glium::texture::SrgbTexture2d::empty(
                ctx, 
                self.width, 
                self.height
            ).unwrap();

            let sampler = SamplerBehavior {
                minify_filter: MinifySamplerFilter::Nearest,
                magnify_filter: MagnifySamplerFilter::Nearest,
                .. Default::default()
            };

            return Ok(
                Box::new(
                    GpuImage::new(gl, sampler)
                )
            )
        }
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