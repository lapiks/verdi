use std::ops::{Deref, DerefMut};

use glium::uniforms::UniformValue;
use mlua::UserData;
use verdi_database::{Resource, ResourceId, Assets, Handle};
use verdi_math::{Vec2, Mat4};

use crate::gpu_image::GpuImage;

pub type UniformId = ResourceId;

pub trait UniformType: 'static {
    fn get_value(&self) -> UniformValue;
}

impl UniformType for f32 {
    fn get_value(&self) -> UniformValue {
        UniformValue::Float(*self)
    }
}

impl UniformType for Vec2 {
    fn get_value(&self) -> UniformValue {
        UniformValue::Vec2(self.to_array())
    }
}

impl UniformType for Mat4 {
    fn get_value(&self) -> UniformValue {
        UniformValue::Mat4(self.to_cols_array_2d())
    }
}

impl UniformType for GpuImage {
    fn get_value(&self) -> UniformValue {
        UniformValue::SrgbTexture2d(&self.gl, Some(self.sampler))
    }
}

impl UniformType for bool {
    fn get_value(&self) -> UniformValue {
        UniformValue::Bool(*self)
    }
}

#[derive(Clone)]
pub struct Uniform<T: UniformType> {
    pub value: T,
}

impl<T: UniformType> Resource for Uniform<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<T: UniformType> Uniform<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }

    pub fn get_value(&self)  -> UniformValue {
        self.get_value()
    }
}

#[derive(Clone)]
pub struct UniformHandle<T: UniformType>(Handle<Uniform<T>>);

impl<T: UniformType> UniformHandle<T> {
    pub fn new(assets: Assets, id: UniformId) -> Self {
        UniformHandle(assets.new_handle(id))
    }
}

impl<T: UniformType> Deref for UniformHandle<T> {
    type Target = Handle<Uniform<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: UniformType> DerefMut for UniformHandle<T> {
      fn deref_mut(&mut self) -> &mut Handle<Uniform<T>> {
        &mut self.0
    }
}

impl<T: UniformType> UserData for UniformHandle<T> {}