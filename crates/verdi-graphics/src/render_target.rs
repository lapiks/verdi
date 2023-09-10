use std::rc::Rc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderTargetError {
    #[error("Texture creation failed")]
    TextureError,
    #[error("Depth buffer creation failed")]
    DepthBufferError,
}

pub struct RenderTarget {
    color_target: miniquad::TextureId,
    depth_target: miniquad::TextureId,
    width: u32,
    height: u32
}

impl RenderTarget {
    pub fn new(ctx: &mut dyn miniquad::RenderingBackend, width: u32, height: u32) -> Result<Self, RenderTargetError> {
        let color_target = ctx.new_render_texture(miniquad::TextureParams {
            width,
            height,
            format: miniquad::TextureFormat::RGBA8,
            ..Default::default()
        });
        let depth_target = ctx.new_render_texture(miniquad::TextureParams {
            width,
            height,
            format: miniquad::TextureFormat::Depth,
            ..Default::default()
        });

        Ok(
            Self {
                color_target,
                depth_target,
                width,
                height,
            }
        )
    }

    pub fn get_color_target(&self) -> miniquad::TextureId {
        self.color_target
    }

    pub fn get_depth_target(&self) -> miniquad::TextureId {
        self.depth_target
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        ( self.width, self.height )
    }
}