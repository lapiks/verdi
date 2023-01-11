use std::sync::{Arc, Mutex};

use glium::{
    uniforms::{
        UniformValue, 
        Uniforms as GliumUniforms, 
    },
    Display
};

use rlua::{UserData, UserDataMethods, prelude::LuaValue};
use slotmap::{new_key_type, Key};

use crate::{
    assets::Assets,
    uniforms::{UniformId, Uniforms}, 
    gpu_assets::{GpuAssets}, 
    program::ProgramId, prelude::GraphicsChip,
};

const MAX_UNIFORMS: usize = 64;

new_key_type! {
    pub struct MaterialId;
}

#[derive(Clone, Copy)]
pub struct Material {
    pub program: ProgramId,
    uniforms: [Option<(&'static str, UniformId)>; MAX_UNIFORMS],
    pub id: MaterialId,
}

impl Material {
    pub fn new(program: ProgramId) -> Self {
        Self {
            program,
            uniforms: [None; MAX_UNIFORMS],
            id: MaterialId::null(),
        }
    }

    pub fn add_uniform(&mut self, name: &'static str, id: UniformId) -> &mut Self {
        for uniform in &mut self.uniforms[..] {
            if uniform.is_none() {
                *uniform = Some((name, id));
                break;
            }
        }
        self
    }

    pub fn get_uniform_values<'a>(&self, uniforms: &'a Uniforms, gpu_assets: &'a GpuAssets) -> Option<UniformValues<'a>> {
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

        Some(UniformValues { 
            //program, 
            uniform_values 
        })
    }

    pub fn prepare_rendering(&self, display: &Display, uniforms: &Uniforms, assets: &Assets, gpu_assets: &mut GpuAssets) {
        for uniform_id in self.uniforms {
            if uniform_id.is_some() {
                match uniform_id.unwrap().1 {
                    UniformId::Texture(_) => {
                        if let Some(texture_uniform) = uniforms.get_texture(uniform_id.unwrap().1) {
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

#[derive(Clone)]
pub struct MaterialRef {
    pub gpu: Arc<Mutex<GraphicsChip>>,
    pub id: MaterialId,
}

impl MaterialRef {
    pub fn new(gpu: Arc<Mutex<GraphicsChip>>, id: MaterialId) -> Self{
        Self { gpu, id }
    }
}

impl UserData for MaterialRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addUniform", |_, material, (name, value): (String, LuaValue)| {
            let mut gpu = material.gpu.lock().unwrap();
            let material = gpu.assets.get_material_mut(material.id).unwrap();

            match value {
                LuaValue::Nil => todo!(),
                LuaValue::Boolean(v) => {
                    material.add_uniform(&name, gpu.uniforms.add_boolean(v));
                },
                LuaValue::LightUserData(_) => todo!(),
                LuaValue::Integer(_) => todo!(),
                LuaValue::Number(v) => {
                    material.add_uniform(&name, gpu.uniforms.add_float(v as f32));
                }
                LuaValue::String(_) => todo!(),
                LuaValue::Table(_) => todo!(),
                LuaValue::Function(_) => todo!(),
                LuaValue::Thread(_) => todo!(),
                LuaValue::UserData(_) => todo!(),
                LuaValue::Error(_) => todo!(),
            };

            Ok(())
        });
    }
}

pub struct UniformValues<'a> {
    //program: &'a GpuProgram,
    uniform_values: [Option<(&'static str, UniformValue<'a>)>; MAX_UNIFORMS],
}

impl<'material> GliumUniforms for UniformValues<'material> {
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