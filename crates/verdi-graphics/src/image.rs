
extern crate image;

struct Image {
    width: u32,
    height: u32,
    image: image::RgbaImage
}

impl Image {
    pub fn new(path: &String) -> Self{
        use std::io::Cursor;

        let image = image::load(
            Cursor::new(path),
            image::ImageFormat::Png
        ).unwrap().to_rgba8();

        let dim = image.dimensions();

        Self { 
            width: dim.0, 
            height: dim.1,
            image: image
        }
    }
}