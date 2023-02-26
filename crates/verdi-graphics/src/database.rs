use crate::{
    assets::Assets, 
    uniforms::Uniforms, 
};

/// Render resources database.
pub struct DataBase {
    pub assets: Assets,
    pub uniforms: Uniforms,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),
            uniforms: Uniforms::new(),
        }
    }
}