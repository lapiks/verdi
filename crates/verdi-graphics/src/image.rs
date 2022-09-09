
use image::{io::Reader as ImageReader, GenericImageView};

pub struct Image {
    width: u32,
    height: u32,
    image: image::DynamicImage
}

impl Image {
    pub fn new(path: &String) -> Self{
        use std::io::Cursor;

        let img = ImageReader::open(path).unwrap().decode().unwrap();

        let dim = img.dimensions();

        Self { 
            width: dim.0, 
            height: dim.1,
            image: img
        }
    }
}