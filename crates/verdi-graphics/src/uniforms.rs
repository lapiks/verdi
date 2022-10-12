use glium::uniforms::{
    UniformValue, 
    SamplerBehavior, 
    MinifySamplerFilter,
    MagnifySamplerFilter
};
use slotmap::{new_key_type, SlotMap};
use verdi_math::{Mat4, Vec2};

use crate::{
    gpu_assets::GpuAssets, 
    image::ImageId
};

new_key_type! {
    pub struct FloatUniformId;
    pub struct Vec2UniformId;
    pub struct Mat4UniformId;
    pub struct TextureUniformId;
}

#[derive(Copy, Clone)]
pub enum UniformId {
    Float(FloatUniformId),
    Vec2(Vec2UniformId),
    Mat4(Mat4UniformId),
    Texture(TextureUniformId),
}

//type UniformList<T> = SlotMap<Uniform, T>;

pub struct Uniforms {
    floats: SlotMap<FloatUniformId, f32>,
    vec2s: SlotMap<Vec2UniformId, Vec2>,
    mat4s: SlotMap<Mat4UniformId, Mat4>,
    textures: SlotMap<TextureUniformId, TextureUniform>,
}

pub struct TextureUniform {
    pub id: ImageId,
    pub sampler: SamplerBehavior,
}

impl TextureUniform {
    pub fn new(id: ImageId) -> Self{
        let sampler = SamplerBehavior {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            .. Default::default()
        };
        
        Self {
            id,
            sampler,
        }
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            floats: SlotMap::default(),
            vec2s: SlotMap::default(),
            mat4s: SlotMap::default(),
            textures: SlotMap::default(),
        }
    }
}

impl Uniforms {
    pub fn get_value<'a>(&'a self, uniform_id: UniformId, gpu_assets: &'a GpuAssets) -> Option<UniformValue> {
        match uniform_id {
            UniformId::Float(uniform_id) => {
                self.floats
                    .get(uniform_id)
                    .map(|&value| UniformValue::Float(value))
            }
            UniformId::Vec2(uniform_id) => {
                self.vec2s
                    .get(uniform_id)
                    .map(|&value| UniformValue::Vec2(value.to_array()))
            }
            UniformId::Mat4(uniform_id) => {
                self.mat4s
                    .get(uniform_id)
                    .map(|&value| UniformValue::Mat4(value.to_cols_array_2d()))
            }
            UniformId::Texture(uniform_id) => {
                if let Some(uniform_texture) = self.textures.get(uniform_id) {
                    let gpu_tex = gpu_assets.get_texture(uniform_texture.id)?;
                    Some(UniformValue::SrgbTexture2d(&gpu_tex.gl, Some(uniform_texture.sampler)))
                }
                else {
                    None
                }
            }
        }
    }

    pub fn add_float(&mut self, value: f32) -> UniformId {
        let id = self.floats.insert(value);

        UniformId::Float(id)
    }

    pub fn get_float_mut(&mut self, id: FloatUniformId) -> Option<&mut f32> {
        self.floats.get_mut(id)
    }

    pub fn add_vec2(&mut self, value: Vec2) -> UniformId {
        let id = self.vec2s.insert(value);

        UniformId::Vec2(id)
    }

    pub fn get_vec2_mut(&mut self, id: Vec2UniformId) -> Option<&mut Vec2> {
        self.vec2s.get_mut(id)
    }

    pub fn add_mat4(&mut self, value: Mat4) -> UniformId {
        let id = self.mat4s.insert(value);

        UniformId::Mat4(id)
    }

    pub fn get_mat4_mut(&mut self, id: UniformId) -> Option<&mut Mat4> {
        match id {
            UniformId::Mat4(id) => {
                self.mat4s.get_mut(id)
            }
            _ => {
                // wrong type
                None
            }
        }
    }

    pub fn add_texture(&mut self, value: TextureUniform) -> UniformId {
        let id = self.textures.insert(value);

        UniformId::Texture(id)
    }

    pub fn get_texture(&self, id: TextureUniformId) -> Option<&TextureUniform> {
        self.textures.get(id)
    }

    pub fn get_texture_mut(&mut self, id: TextureUniformId) -> Option<&mut TextureUniform> {
        self.textures.get_mut(id)
    }
}