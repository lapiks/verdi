use glium::{uniforms::{
    UniformValue, 
    Uniforms as GliumUniforms, 
}, Display};
use uuid::Uuid;

use crate::{
    assets::{AssetId, Assets},
    uniforms::{UniformId, Uniforms}, 
    gpu_assets::{GpuAssets},
};

const MAX_UNIFORMS: usize = 64;

pub struct Material {
    program: AssetId,
    uniforms: [Option<(&'static str, UniformId)>; MAX_UNIFORMS],
    pub id: AssetId,
}

impl Material {
    pub fn new(program: AssetId) -> Self {
        Self {
            program,
            uniforms: [None; MAX_UNIFORMS],
            id: Uuid::nil(),
        }
    }

    pub fn add_uniform(&mut self, name: &'static str, id: UniformId) {
        for uniform in &mut self.uniforms[..] {
            if uniform.is_none() {
                *uniform = Some((name, id));
                break;
            }
        }
    }

    pub fn get_ref<'a>(&self, uniforms: &'a Uniforms, gpu_assets: &'a GpuAssets) -> Option<MaterialRef<'a>> {
        // construct uniform values from the material uniforms description 
        let mut uniform_values = [None; MAX_UNIFORMS];
        for (uniform_value, uniform_id) in uniform_values.iter_mut().zip(self.uniforms) {
            if let Some((name, id)) = uniform_id {
                if let Some(value) = uniforms.get_value(id, gpu_assets) {
                    *uniform_value = Some((name, value));
                }
                else {
                    // missing uniform
                    return None;
                }
            }
            else {
                break;
            }
        }

        //let program = gpu_assets.get_program(self.program)?;

        Some(MaterialRef { 
            //program, 
            uniform_values 
        })
    }

    pub fn prepare_rendering(&self, display: &Display, uniforms: &Uniforms, assets: &Assets, gpu_assets: &mut GpuAssets) {
        for uniform_id in self.uniforms {
            if uniform_id.is_some() {
                match uniform_id.unwrap().1 {
                    UniformId::Texture(id) => {
                        if let Some(texture_uniform) = uniforms.get_texture(id) {
                            if let Some(texture) = assets.get_texture(texture_uniform.id) {
                                texture.prepare_rendering(display, gpu_assets);
                            }
                        }
                    },
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
}

pub struct MaterialRef<'a> {
    //program: &'a GpuProgram,
    uniform_values: [Option<(&'static str, UniformValue<'a>)>; MAX_UNIFORMS],
}

impl<'material> GliumUniforms for MaterialRef<'material> {
    fn visit_values<'a, F>(&'a self, mut set_uniform: F)
    where
        F: FnMut(&str, UniformValue<'a>),
    {
        for uniform in &self.uniform_values[..] {
            if let Some((name, value)) = *uniform {
                set_uniform(name, value);
            }
        }
    }
}