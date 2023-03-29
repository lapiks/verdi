use std::{rc::Rc, cell::RefCell};

use glium::{
    uniforms::{
        UniformValue, 
        Uniforms as GliumUniforms, 
    },
    Display
};

use mlua::{UserData, UserDataMethods, prelude::LuaValue};
use slotmap::{new_key_type, Key};

use crate::{
    assets::Assets,
    uniforms::{UniformId, Uniforms}, 
    gpu_assets::{GpuAssets}, 
    program::ProgramId, prelude::GraphicsChip, globals::GlobalUniforms, pass::Pass,
};

const MAX_UNIFORMS: usize = 64;

new_key_type! {
    pub struct MaterialId;
}

/// A material defines the program and uniforms to use when rendering a mesh.
#[derive(Clone)]
pub struct Material {
    pub program: ProgramId,
    uniforms: Vec<Option<(String, UniformId)>>,
    pub id: MaterialId,
}

impl Material {
    pub fn new(program: ProgramId, global_uniforms: &GlobalUniforms) -> Self {
        let mut uniforms = vec![None; MAX_UNIFORMS];
        // add global uniforms to the material
        uniforms[0] = Some(("u_model".to_string(), global_uniforms.model_matrix));
        uniforms[1] = Some(("u_view".to_string(), global_uniforms.view_matrix));
        uniforms[2] = Some(("u_projection".to_string(), global_uniforms.projection_matrix));
        uniforms[3] = Some(("u_resolution".to_string(), global_uniforms.resolution));

        Self {
            program,
            uniforms,
            id: MaterialId::null(),
        }
    }

    pub fn add_uniform(&mut self, name: &str, id: UniformId) -> &mut Self {
        for uniform in &mut self.uniforms[..] {
            if uniform.is_none() {
                *uniform = Some((name.to_string(), id));
                break;
            }
        }
        self
    }

    pub fn get_uniform_values<'a>(&'a self, uniforms: &'a Uniforms, gpu_assets: &'a GpuAssets, pass: &Pass) -> Option<UniformValues<'a>> {
        // construct uniform values from the material uniforms description 
        let mut uniform_values = [None; MAX_UNIFORMS];
        for (uniform_value, uniform_id) in uniform_values.iter_mut().zip(&self.uniforms) {
            if let Some((name, id)) = uniform_id {
                if let Some(value) = uniforms.get_value(*id, gpu_assets) {
                    *uniform_value = Some((&name[..], value));
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
        for uniform_id in &self.uniforms {
            if uniform_id.is_some() {
                match uniform_id.as_ref().unwrap().1 {
                    UniformId::Texture(_) => {
                        if let Some(texture_uniform) = uniforms.get_texture(uniform_id.as_ref().unwrap().1) {
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
pub struct MaterialHandle {
    pub gpu: Rc<RefCell<GraphicsChip>>,
    pub id: MaterialId,
}

impl MaterialHandle {
    pub fn new(gpu: Rc<RefCell<GraphicsChip>>, id: MaterialId) -> Self{
        Self { gpu, id }
    }
}

impl UserData for MaterialHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addUniform", |_, material, (name, value): (String, LuaValue)| {
            let mut uniform_id = None;
            {
                let gpu = material.gpu.borrow_mut();
                match value {
                    LuaValue::Nil => todo!(),
                    LuaValue::Boolean(v) => {
                        uniform_id = Some(gpu.database.borrow_mut().uniforms.add_boolean(v));
                    },
                    LuaValue::LightUserData(_) => todo!(),
                    LuaValue::Integer(_) => todo!(),
                    LuaValue::Number(v) => {
                        uniform_id = Some(gpu.database.borrow_mut().uniforms.add_float(v as f32));
                    }
                    LuaValue::String(_) => todo!(),
                    LuaValue::Table(_) => todo!(),
                    LuaValue::Function(_) => todo!(),
                    LuaValue::Thread(_) => todo!(),
                    LuaValue::UserData(_) => todo!(),
                    LuaValue::Error(_) => todo!(),
                };
            }
    
            if let Some(uniform_id) = uniform_id {
                let gpu = material.gpu.borrow();
                let material = gpu.database.borrow_mut().assets
                    .get_material_mut(material.id).unwrap()
                    .add_uniform(&name, uniform_id);
            }
    
            Ok(())
        });
    }
}

pub struct UniformValues<'a> {
    //program: &'a GpuProgram,
    uniform_values: [Option<(&'a str, UniformValue<'a>)>; MAX_UNIFORMS],
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