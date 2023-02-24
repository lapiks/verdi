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
        let mut assets = Assets::new();
        let mut uniforms = Uniforms::new();

        Self {
            assets,
            uniforms,
        }
    }
}