use std::ops::{Deref, DerefMut};

use mlua::{UserData, UserDataMethods, prelude::LuaValue};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    program::{ProgramId, Program}, 
    globals::GlobalUniforms, 
    image::ImageId, 
    uniform::{Uniform, UniformHandle, UniformValue}, gpu_assets::{PrepareAsset, GpuAssets, GpuAsset, GpuAssetError}, shader::Shader, gpu_program::GpuProgram, 
};

pub type MaterialId = ResourceId;

/// A material defines the program and uniforms to use when rendering a mesh.
#[derive(Clone)]
pub struct Material {
    pub program: ProgramId,
    textures: Vec<ImageId>,
    uniforms: Vec<(String, UniformHandle)>,
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
        let mut uniforms = Vec::with_capacity(4);
        // add global uniforms to the material
        uniforms.push(("u_model".to_string(), global_uniforms.model_matrix.clone()));
        uniforms.push(("u_view".to_string(), global_uniforms.view_matrix.clone()));
        uniforms.push(("u_projection".to_string(), global_uniforms.projection_matrix.clone()));
        uniforms.push(("u_resolution".to_string(), global_uniforms.resolution.clone()));

        Self {
            program,
            textures: Vec::default(),
            uniforms,
            id: MaterialId::null(),
        }
    }

    pub fn add_uniform(&mut self, name: String, uniform_handle: UniformHandle) -> &mut Self {
        self.uniforms.push((name, uniform_handle));

        self
    }

    pub fn add_texture(&mut self, texture: ImageId) {
        self.textures.push(texture);
    }

    pub fn get_textures(&self) -> &Vec<ImageId> {
        &self.textures
    }

    pub fn get_uniform_values(&self) -> Option<Vec<u8>> {
        // construct a buffer of uniform values from the material uniforms description as buffer of bytes
        let mut uniform_values = Vec::default();

        for (_, uniform_handle) in self.uniforms.iter() {
            if let Some(uniform) = uniform_handle.get_datas().get::<Uniform>(uniform_handle.get_id()) {
                uniform_values.append(&mut uniform.encode_u8());
            }
            else {
                // missing uniform
                return None;
            }
        }

        Some(uniform_values)
    }
}

impl PrepareAsset for Material {
    fn prepare_rendering(&self, ctx: &mut dyn miniquad::RenderingBackend, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        if let Some(program) = assets.get_datas().get::<Program>(self.program) {
            if let Some(vs) = assets.get_datas().get::<Shader>(program.vs) {
                if let Some(fs) = assets.get_datas().get::<Shader>(program.fs) {
                    let mut uniform_descs = Vec::with_capacity(self.uniforms.len());
                    for uniform in &self.uniforms {
                        if let Some(uniform_value) = assets.get_datas().get::<Uniform>(uniform.1.get_id())
                        {
                            uniform_descs.push(
                                miniquad::UniformDesc::new(
                                    uniform.0.as_str(),
                                    uniform_value.get_quad_type(),
                                )
                            );
                        }
                    }

                    let shader_meta = miniquad::ShaderMeta {
                        images: vec!["u_texture".to_string()], // TODO
                        uniforms: miniquad::UniformBlockLayout {
                            uniforms: uniform_descs,
                        },
                    };

                    let shader = ctx.new_shader(
                        miniquad::ShaderSource::Glsl { 
                            vertex: vs.get_source(), 
                            fragment: fs.get_source() 
                        },
                        shader_meta
                    )?;
    
                    return Ok(
                        Box::new(
                            GpuProgram::new(shader)
                        )
                    );
                }
            }
        }
        
        Err(GpuAssetError::PreparationFailed)
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