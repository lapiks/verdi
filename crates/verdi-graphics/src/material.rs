use crate::image::ImageRef;

pub struct Material {
    pub texture: Option<ImageRef>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            texture: Option::default()
        }
    }
}