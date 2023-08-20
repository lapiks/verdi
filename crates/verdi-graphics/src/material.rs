use std::{rc::Rc, cell::RefCell, ops::{Deref, DerefMut}};

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
    uniform::{Uniform, UniformId}, 
};

const MAX_UNIFORMS: usize = 64;

pub type MaterialId = ResourceId;

/// A material defines the program and uniforms to use when rendering a mesh.
#[derive(Clone)]
pub struct Material {
    pub program: ProgramId,
    textures: Vec<ImageId>,
    uniforms: Vec<Option<(String, UniformId)>>,
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
        uniforms[0] = Some(("u_model".to_string(), global_uniforms.model_matrix));
        uniforms[1] = Some(("u_view".to_string(), global_uniforms.view_matrix));
        uniforms[2] = Some(("u_projection".to_string(), global_uniforms.projection_matrix));
        uniforms[3] = Some(("u_resolution".to_string(), global_uniforms.resolution));

        Self {
            program,
            textures: Vec::default(),
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

    pub fn get_textures(&self) -> &Vec<ImageId> {
        &self.textures
    }

    pub fn get_uniform_values(&self, assets: &Assets, pass: &Pass) -> Option<UniformValues> {
        // construct uniform values from the material uniforms description 
        let mut uniform_values = [None; MAX_UNIFORMS];

        // TODO: fix ça
        // for (uniform_value, uniform_id) in uniform_values.iter_mut().zip(&self.uniforms) {
        //     if let Some((name, id)) = uniform_id {
        //         if let Some(value) = assets.get::<Uniform<f32>>(*id) {
        //             *uniform_value = Some((&name[..], value.get_value()));
        //         }
        //         else {
        //             // missing uniform
        //             return None;
        //         }
        //     }
        //     else {
        //         break;
        //     }
        // }

        //let program = gpu_resources.get_program(self.program)?;

        Some(UniformValues { 
            //program, 
            uniform_values 
        })
    }
}

pub struct MaterialHandle(Handle<Material>);

impl Deref for MaterialHandle {
    type Target = Handle<Material>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialHandle {
    fn deref_mut(&mut self) -> &mut Handle<Material> {
        &mut self.0
    }
}

impl MaterialHandle {
    pub fn new(assets: Rc<RefCell<Assets>>, id: MaterialId) -> Self{
        Self(Handle::new(assets, id))
    }
}

impl UserData for MaterialHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addUniform", |_, material, (name, value): (String, LuaValue)| {
            let mut uniform_id = None;
            {
                let mut assets = material.get_assets_mut();
                match value {
                    LuaValue::Nil => todo!(),
                    LuaValue::Boolean(v) => {
                        uniform_id = Some(assets.add(Box::new(Uniform::new(v))));
                    },
                    LuaValue::LightUserData(_) => todo!(),
                    LuaValue::Integer(_) => todo!(),
                    LuaValue::Number(v) => {
                        uniform_id = Some(assets.add(Box::new(Uniform::new(v as f32))));
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
                let material_id = material.get_id();
                let mut assets = material.get_assets_mut();
                let material = assets
                    .get_mut::<Material>(material_id)
                    .unwrap()
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