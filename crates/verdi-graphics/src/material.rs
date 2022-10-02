use glium::uniforms::{
    UniformValue, 
    Uniforms as GliumUniforms, 
    AsUniformValue
};

use crate::{
    assets::AssetId,
    uniforms::{UniformId, Uniforms}, 
    gpu_assets::{GpuAssets},
};

const MAX_UNIFORMS: usize = 64;

pub struct Material {
    program: AssetId,
    uniforms: [Option<(&'static str, UniformId)>; MAX_UNIFORMS],
}

impl Material {
    pub fn new(program: AssetId) -> Self {
        Self {
            program,
            uniforms: [None; MAX_UNIFORMS],
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
}

pub struct MaterialRef<'a> {
    //program: &'a GpuProgram,
    uniform_values: [Option<(&'static str, UniformValue<'a>)>; MAX_UNIFORMS],
}

impl<'a> MaterialRef<'a> {
    pub fn add_uniform<T: AsUniformValue>(&mut self, name: &str, value: &T) {

    }
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