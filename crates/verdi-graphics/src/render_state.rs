use verdi_math::{Vec4, Mat4};

pub struct RenderState {
    pub clear_color: Vec4,
    pub view: Mat4,
    pub enable_lighting: bool,
    pub enable_fog: bool,
    pub fog_start: f32,
    pub fog_end: f32,
}

/// A struct defining some global render state.
impl RenderState {
    pub fn new() -> Self {
        Self {
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
            view: Mat4::IDENTITY,
            enable_lighting: true,
            enable_fog: false,
            fog_start: 0.0, 
            fog_end: 0.0,
        }
    }
}