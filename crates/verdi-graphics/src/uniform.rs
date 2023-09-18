use std::ops::{Deref, DerefMut};

use mlua::UserData;
use verdi_database::{Resource, ResourceId, Assets, Handle};
use verdi_math::{Vec2, Mat4, Vec3, Vec4};

use crate::{gpu_image::GpuImage, gpu_assets::GpuAssets, image::ImageId};

pub type UniformId = ResourceId;


#[derive(Clone, Copy)]
pub enum UniformValue {
    Bool(bool),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat4(Mat4),
    Texture(ImageId),
}

impl UniformValue {
    fn get_gl_value<'a>(&'a self, gpu_assets: &'a GpuAssets) -> glium::uniforms::UniformValue {
        match self {
            UniformValue::Bool(value) => value.get_gl_value(gpu_assets),
            UniformValue::Float(value) => value.get_gl_value(gpu_assets),
            UniformValue::Vec2(value) =>  value.get_gl_value(gpu_assets),
            UniformValue::Vec3(value) =>  value.get_gl_value(gpu_assets),
            UniformValue::Vec4(value) =>  value.get_gl_value(gpu_assets),
            UniformValue::Mat4(value) =>  value.get_gl_value(gpu_assets),
            UniformValue::Texture(value) =>  value.get_gl_value(gpu_assets),
        }
    }
}

pub trait UniformsLayout {}

pub trait UniformType: 'static {
    fn get_gl_value<'a>(&'a self, _:  &'a GpuAssets) -> glium::uniforms::UniformValue;
}

impl UniformType for f32 {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Float(*self)
    }
}

impl UniformType for Vec2 {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec2(self.to_array())
    }
}

impl UniformType for Vec3 {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec3(self.to_array())
    }
}

impl UniformType for Vec4 {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec4(self.to_array())
    }
}

impl UniformType for Mat4 {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Mat4(self.to_cols_array_2d())
    }
}

impl UniformType for bool {
    fn get_gl_value<'a>(&'a self, _: &'a GpuAssets) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Bool(*self)
    }
}

impl UniformType for ImageId {
    fn get_gl_value<'a>(&'a self, gpu_assets: &'a GpuAssets) -> glium::uniforms::UniformValue  {
        let gpu_image = gpu_assets.get::<GpuImage>(*self).expect("Gpu Image not Found");
        glium::uniforms::UniformValue::SrgbTexture2d(&gpu_image.get_gl_texture(), Some(*gpu_image.get_gl_sampler()))
    }
}

//#[derive(Clone)]
pub struct Uniform {
    pub value: UniformValue,
}

impl Resource for Uniform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Uniform {
    pub fn new(value: UniformValue) -> Self {
        Self {
            value
        }
    }

    pub fn get_value(&self) -> &UniformValue {
        &self.value
    }
 
    pub fn get_gl_value<'a>(&'a self, gpu_assets: &'a GpuAssets) -> glium::uniforms::UniformValue {
        self.value.get_gl_value(gpu_assets)
    }
 }

#[derive(Clone)]
pub struct UniformHandle(Handle);

impl UniformHandle {
    pub fn new(assets: Assets, id: UniformId) -> Self {
        UniformHandle(assets.new_handle(id))
    }
}

impl Deref for UniformHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UniformHandle {
      fn deref_mut(&mut self) -> &mut Handle {
        &mut self.0
    }
}

impl UserData for UniformHandle {}