use verdi_math::Vec4;

use crate::uniforms::Uniforms;

pub struct RenderState {
    pub clear_color: Vec4,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}