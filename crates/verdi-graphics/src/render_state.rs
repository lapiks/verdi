use verdi_math::Vec4;

pub struct RenderState {
    pub clear_color: Vec4,
}

/// A struct defining some global render state.
impl RenderState {
    pub fn new() -> Self {
        Self {
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}