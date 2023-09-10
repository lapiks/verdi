use verdi_database::Resource;

use crate::gpu_assets::GpuAsset;

pub struct GpuProgram(miniquad::ShaderId);

impl GpuProgram {
    pub fn new(id: miniquad::ShaderId) -> Self {
        Self(id)
    }
}

impl GpuProgram {
    pub fn get_shader(&self) -> miniquad::ShaderId {
        self.0
    }
}

impl Resource for GpuProgram {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuProgram {}