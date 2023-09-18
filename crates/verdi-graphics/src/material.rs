use std::ops::{Deref, DerefMut};

use mlua::{UserData, UserDataMethods, prelude::LuaValue};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    program::ProgramHandle, 
    globals::GlobalUniforms, 
    uniform::{Uniform, UniformHandle, UniformValue},
};

const MAX_UNIFORMS: usize = 64;

pub type MaterialId = ResourceId;

/// A material defines the program and uniforms to use when rendering a mesh.
#[derive(Clone)]
pub struct Material {
    pub program: ProgramHandle,
    uniforms: Vec<Option<(&'static str, UniformHandle)>>,
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
    pub fn new(program: ProgramHandle, global_uniforms: &GlobalUniforms) -> Self {
        let mut uniforms = vec![None; MAX_UNIFORMS];
        // add global uniforms to the material
        uniforms[0] = Some(("u_model", global_uniforms.model_matrix.clone()));
        uniforms[1] = Some(("u_view", global_uniforms.view_matrix.clone()));
        uniforms[2] = Some(("u_projection", global_uniforms.projection_matrix.clone()));
        uniforms[3] = Some(("u_resolution", global_uniforms.resolution.clone()));

        Self {
            program,
            uniforms,
            id: MaterialId::null(),
        }
    }

    pub fn add_uniform(&mut self, name: &'static str, uniform_handle: UniformHandle) -> &mut Self {
        for uniform in &mut self.uniforms[..] {
            if uniform.is_none() {
                *uniform = Some((name, uniform_handle));
                break;
            }
        }
        self
    }

    pub fn get_uniforms(&self) -> &Vec<Option<(&'static str, UniformHandle)>> {
        &self.uniforms
    }
}

pub struct GlUniformValues<'a> {
    pub uniform_values: [Option<(&'a str, glium::uniforms::UniformValue<'a>)>; MAX_UNIFORMS],
}

impl<'material> glium::uniforms::Uniforms for GlUniformValues<'material> {
    fn visit_values<'a, F>(&'a self, mut set_uniform: F)
    where
        F: FnMut(&str, glium::uniforms::UniformValue<'a>),
    {
        for uniform in &self.uniform_values[..] {
            if let Some((name, value)) = *uniform {
                set_uniform(name, value);
            }
            else {
                break;
            }
        }
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
                        uniform = Some(
                            UniformHandle::new(
                                assets.clone(), 
                                assets.add(
                                    Box::new(
                                        Uniform::new(
                                            UniformValue::Bool(v)
                                        )
                                    )
                                )
                            )
                        );
                    },
                    LuaValue::LightUserData(_) => todo!(),
                    LuaValue::Integer(_) => todo!(),
                    LuaValue::Number(v) => {
                        uniform = Some(
                            UniformHandle::new(
                                assets.clone(), 
                                assets.add(
                                    Box::new(
                                        Uniform::new(
                                            UniformValue::Float(v as f32)
                                        )
                                    )
                                )
                            )
                        );
                    }
                    LuaValue::String(_) => todo!(),
                    LuaValue::Table(_) => todo!(),
                    LuaValue::Function(_) => todo!(),
                    LuaValue::Thread(_) => todo!(),
                    LuaValue::UserData(_) => todo!(),
                    LuaValue::Error(_) => todo!(),
                };
            }
    
            // TODO: commented because name should be a static &str and is a String here 
            // if let Some(uniform) = uniform {
            //     let material_id = material.get_id();
            //     let mut assets = material.get_datas_mut();
            //     let material = assets
            //         .get_mut::<Material>(material_id)
            //         .unwrap()
            //         .add_uniform(name, uniform);
            // }
    
            Ok(())
        });
    }
}