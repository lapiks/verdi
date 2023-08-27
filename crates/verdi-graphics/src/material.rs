use std::ops::{Deref, DerefMut};

use glium::
    uniforms::{UniformValue, Uniforms as GliumUniforms}
;

use mlua::{UserData, UserDataMethods, prelude::LuaValue};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    program::ProgramId, 
    globals::GlobalUniforms, 
    pass::Pass, 
    image::ImageId, 
    uniform::{Uniform, UniformHandle}, 
};

const MAX_UNIFORMS: usize = 64;

pub type MaterialId = ResourceId;

/// A material defines the program and uniforms to use when rendering a mesh.
#[derive(Clone)]
pub struct Material {
    pub program: ProgramId,
    textures: Vec<ImageId>,
    uniforms: Vec<Option<(String, UniformHandle)>>,
    pub id: MaterialId,
}

impl Resource for Material {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Material {
    pub fn new(program: ProgramId, global_uniforms: &GlobalUniforms) -> Self {
        let mut uniforms = vec![None; MAX_UNIFORMS];
        // add global uniforms to the material
        uniforms[0] = Some(("u_model".to_string(), global_uniforms.model_matrix.clone()));
        uniforms[1] = Some(("u_view".to_string(), global_uniforms.view_matrix.clone()));
        uniforms[2] = Some(("u_projection".to_string(), global_uniforms.projection_matrix.clone()));
        uniforms[3] = Some(("u_resolution".to_string(), global_uniforms.resolution.clone()));

        Self {
            program,
            textures: Vec::default(),
            uniforms,
            id: MaterialId::null(),
        }
    }

    pub fn add_uniform(&mut self, name: String, uniform_handle: UniformHandle) -> &mut Self {
        for uniform in &mut self.uniforms[..] {
            if uniform.is_none() {
                *uniform = Some((name, uniform_handle));
                break;
            }
        }
        self
    }

    pub fn get_textures(&self) -> &Vec<ImageId> {
        &self.textures
    }

    pub fn get_uniform_values(&self) -> Option<UniformValues> {
        // construct uniform values from the material uniforms description 
        let mut uniform_values = [None; MAX_UNIFORMS];

        for (uniform_value, uniform_handle) in uniform_values.iter_mut().zip(self.uniforms.clone()) {
            if let Some((name, handle)) = uniform_handle.clone() {
                let asset_datas = handle.get_datas();
                if let Some(uniform) = asset_datas.get::<Uniform<f32>>(handle.get_id()) {
                    *uniform_value = Some((name.as_str(), uniform.get_value()));
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

        //let program = gpu_resources.get_program(self.program)?;

        Some(UniformValues { 
            //program, 
            uniform_values 
        })
    }
}

pub struct MaterialHandle(Handle);

impl Deref for MaterialHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialHandle {
    fn deref_mut(&mut self) -> &mut Handle {
        &mut self.0
    }
}

impl MaterialHandle {
    pub fn new(assets: Assets, id: MaterialId) -> Self{
        MaterialHandle(assets.new_handle(id))
    }
}

impl UserData for MaterialHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addUniform", |_, material, (name, value): (String, LuaValue)| {
            let mut uniform = None;
            {
                let assets = material.get_assets_mut();
                match value {
                    LuaValue::Nil => todo!(),
                    LuaValue::Boolean(v) => {
                        uniform = Some(UniformHandle::new(assets.clone(), assets.add(Box::new(Uniform::new(v)))));
                    },
                    LuaValue::LightUserData(_) => todo!(),
                    LuaValue::Integer(_) => todo!(),
                    LuaValue::Number(v) => {
                        uniform = Some(UniformHandle::new(assets.clone(), assets.add(Box::new(Uniform::new(v as f32)))));
                    }
                    LuaValue::String(_) => todo!(),
                    LuaValue::Table(_) => todo!(),
                    LuaValue::Function(_) => todo!(),
                    LuaValue::Thread(_) => todo!(),
                    LuaValue::UserData(_) => todo!(),
                    LuaValue::Error(_) => todo!(),
                };
            }
    
            if let Some(uniform) = uniform {
                let material_id = material.get_id();
                let mut assets = material.get_datas_mut();
                let material = assets
                    .get_mut::<Material>(material_id)
                    .unwrap()
                    .add_uniform(name, uniform);
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