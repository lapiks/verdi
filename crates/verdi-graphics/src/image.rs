
use image::{io::Reader as ImageReader, RgbaImage, ImageError};
use rlua::UserData;

use crate::assets::{AssetId, AssetState};

pub struct Image {
    width: u32,
    height: u32,
    data: RgbaImage,
    state: AssetState,
}

impl Image {
    pub fn new(path: &String) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?;
        let img2 = img.to_rgba8();

        let dim = img2.dimensions();

        Ok(Self { 
            width: dim.0, 
            height: dim.1,
            data: img2,
            state: AssetState::Created
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