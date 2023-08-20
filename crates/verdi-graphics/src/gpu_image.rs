use glium::{Display, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter}};
use verdi_database::Resource;

use crate::{image::Image, gpu_assets::GpuAsset};

pub struct GpuImage {
    pub gl: glium::texture::SrgbTexture2d,
    pub sampler: SamplerBehavior,
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

        // TODO: to be settable
        let sampler = SamplerBehavior {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            .. Default::default()
        };

        Self {
            gl: texture,
            sampler,
        }
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