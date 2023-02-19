use crate::{
    assets::Assets, 
    uniforms::Uniforms, 
    globals::Globals
};


/// Render resources database.
pub struct DataBase{
    pub assets: Assets,
    pub uniforms: Uniforms,
    pub globals: Globals,
}

impl DataBase {
    pub fn new() -> Self {
        let mut assets = Assets::new();
        let mut uniforms = Uniforms::new();
        let globals = Globals::new(&mut assets, &mut uniforms).expect("TODO: enlever le expect");

        Self {
            assets,
            uniforms,
            globals,
        }
    }
}