use std::rc::Rc;

use glium::{
    framebuffer::DepthRenderBuffer, 
    Display, 
    texture::{TextureCreationError, buffer_texture::CreationError, SrgbTexture2d}
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderTargetError {
    #[error("Texture creation failed")]
    TextureError(#[from] TextureCreationError),
    #[error("Depth buffer creation failed")]
    DepthBufferError(#[from] CreationError),
}

pub struct RenderTarget {
    color_target: Rc<SrgbTexture2d>,
    depth_target: DepthRenderBuffer,
    width: u32,
    height: u32
}

impl RenderTarget {
    pub fn new(display: &Display, width: u32, height: u32) -> Result<Self, RenderTargetError> {
        let color_target = SrgbTexture2d::empty(
            display, 
            width, 
            height
        ).unwrap();

        let depth_target = DepthRenderBuffer::new(
            display,
            glium::texture::DepthFormat::I24,
            width,
            height
        ).unwrap();

        Ok(
            Self {
                color_target: Rc::new(color_target),
                depth_target,
                width,
                height,
            }
        )
    }

    pub fn get_color_target(&self) -> Rc<SrgbTexture2d> {
        self.color_target.clone()
    }

    pub fn get_depth_target(&self) -> &DepthRenderBuffer {
        &self.depth_target
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        ( self.width, self.height )
    }
}