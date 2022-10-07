use glium::Display;

use crate::image::Image;

pub struct GpuImage {
    pub gl: glium::texture::SrgbTexture2d,
}

impl GpuImage {
    pub fn new(display: &Display, image: &Image) -> Self {
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &image.get_data().as_raw(), 
            image.get_dimensions()
        );
        let texture = glium::texture::SrgbTexture2d::new(
            display, 
            raw_image
        ).unwrap();

        Self {
            gl: texture
        }
    }
}