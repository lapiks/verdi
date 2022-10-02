use std::collections::HashMap;

use glium::uniforms::UniformValue;
use uuid::Uuid;
use verdi_math::{Mat4, Vec2};

use crate::{
    assets::AssetId, 
    gpu_assets::GpuAssets
};

#[derive(Copy, Clone)]
pub enum UniformId {
    Float(AssetId),
    Vec2(AssetId),
    Mat4(AssetId),
    Texture(AssetId),
}

pub struct Uniforms {
    floats: UniformList<f32>,
    vec2s: UniformList<Vec2>,
    mat4s: UniformList<Mat4>,
    textures: UniformList<AssetId>,
}

type UniformList<T> = HashMap<AssetId, T>;

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            floats: UniformList::new(),
            vec2s: UniformList::new(),
            mat4s: UniformList::new(),
            textures: UniformList::new(),
        }
    }
}

impl Uniforms {
    pub fn get_value(&self, id: UniformId, gpu_assets: &GpuAssets) -> Option<UniformValue> {
        match id {
            UniformId::Float(id) => {
                self.floats
                    .get(&id)
                    .map(|&value| UniformValue::Float(value))
            }
            UniformId::Vec2(id) => {
                self.vec2s
                    .get(&id)
                    .map(|&value| UniformValue::Vec2(value.to_array()))
            }
            UniformId::Mat4(id) => {
                self.mat4s
                    .get(&id)
                    .map(|&value| UniformValue::Mat4(value.to_cols_array_2d()))
            }
            UniformId::Texture(id) => {
                // let gpu_tex = gpu_assets.get_texture(id)?;
                // self.textures
                //     .get(&id)
                //     .map(|&value| UniformValue::SrgbTexture2d(&gpu_tex, None))
                None
            }
        }
    }

    pub fn add_float(&mut self, value: f32) -> UniformId {
        let id = Uuid::new_v4();
        self.floats.insert(id, value);

        UniformId::Float(id)
    }

    pub fn get_float_mut(&mut self, id: AssetId) -> Option<&mut f32> {
        self.floats.get_mut(&id)
    }

    pub fn add_vec2(&mut self, value: Vec2) -> UniformId {
        let id = Uuid::new_v4();
        self.vec2s.insert(id, value);

        UniformId::Vec2(id)
    }

    pub fn get_vec2_mut(&mut self, id: AssetId) -> Option<&mut Vec2> {
        self.vec2s.get_mut(&id)
    }

    pub fn add_mat4(&mut self, value: Mat4) -> UniformId {
        let id = Uuid::new_v4();
        self.mat4s.insert(id, value);

        UniformId::Mat4(id)
    }

    pub fn get_mat4_mut(&mut self, id: UniformId) -> Option<&mut Mat4> {
        match id {
            UniformId::Mat4(id) => {
                self.mat4s.get_mut(&id)
            }
            _ => {
                // wrong type
                None
            }
        }
    }

    pub fn add_texture(&mut self, value: AssetId) -> UniformId {
        let id = Uuid::new_v4();
        self.textures.insert(id, value);

        UniformId::Texture(id)
    }

    pub fn get_texture_mut(&mut self, id: AssetId) -> Option<&mut AssetId> {
        self.textures.get_mut(&id)
    }
}