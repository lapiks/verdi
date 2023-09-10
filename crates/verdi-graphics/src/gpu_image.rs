use verdi_database::Resource;

use crate::{image::Image, gpu_assets::GpuAsset};

pub struct GpuImage(miniquad::TextureId);

impl GpuImage {
    pub fn new(ctx: &mut dyn miniquad::RenderingBackend, image: &Image) -> Self {
        Self(
            ctx.new_texture_from_rgba8(
                image.get_width() as u16,
                image.get_height() as u16,
                &image.get_data().as_raw(),
            )
        )
    }

    pub fn get_quad_texture(&self) -> miniquad::TextureId {
        self.0
    }
}

impl Resource for GpuImage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuImage {}