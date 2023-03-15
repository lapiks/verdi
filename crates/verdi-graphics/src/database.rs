use crate::{
    assets::Assets, 
    uniforms::Uniforms, 
};

/// Render resources database.
pub struct Database {
    pub assets: Assets,
    pub uniforms: Uniforms,
}

impl Database {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),
            uniforms: Uniforms::new(),
        }
    }
}