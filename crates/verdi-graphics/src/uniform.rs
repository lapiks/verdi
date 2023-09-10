use std::ops::{Deref, DerefMut};

use mlua::UserData;
use verdi_database::{Resource, ResourceId, Assets, Handle};
use verdi_math::{Vec2, Mat4, Vec3, Vec4};

pub type UniformId = ResourceId;


#[derive(Clone, Copy)]
pub enum UniformValue {
    Bool(bool),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat4(Mat4),
}

impl UniformValue {
    fn get_quad_type(&self) -> miniquad::UniformType {
        match self {
            UniformValue::Bool(_) => miniquad::UniformType::Int1,
            UniformValue::Float(_) => miniquad::UniformType::Float1,
            UniformValue::Vec2(_) =>  miniquad::UniformType::Float2,
            UniformValue::Vec3(_) =>  miniquad::UniformType::Float3,
            UniformValue::Vec4(_) =>  miniquad::UniformType::Float4,
            UniformValue::Mat4(_) =>  miniquad::UniformType::Mat4,
        }
    }

    fn encode_u8(&self) -> Vec<u8> {
        match self {
            UniformValue::Bool(value) => value.encode_u8(),
            UniformValue::Float(value) => value.encode_u8(),
            UniformValue::Vec2(value) =>  value.encode_u8(),
            UniformValue::Vec3(value) =>  value.encode_u8(),
            UniformValue::Vec4(value) =>  value.encode_u8(),
            UniformValue::Mat4(value) =>  value.encode_u8(),
        }
    }
}

pub trait UniformsLayout {}

pub trait UniformType: 'static {
    fn get_quad_type(&self) -> miniquad::UniformType;
    fn encode_u8(&self) -> Vec<u8>;
}

impl UniformType for f32 {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Float1
    }

    fn encode_u8(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl UniformType for Vec2 {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Float2
    }

    fn encode_u8(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::default();
        for float in self.to_array() {
            res.extend_from_slice(&float.to_le_bytes());
        }
        res
    }    
}

impl UniformType for Vec3 {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Float3
    }

    fn encode_u8(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::default();
        for float in self.to_array() {
            res.extend_from_slice(&float.to_le_bytes());
        }
        res
    }    
}

impl UniformType for Vec4 {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Float4
    }

    fn encode_u8(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::default();
        for float in self.to_array() {
            res.extend_from_slice(&float.to_le_bytes());
        }
        res
    }    
}

impl UniformType for Mat4 {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Mat4
    }

    fn encode_u8(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::default();
        for float in self.to_cols_array() {
            res.extend_from_slice(&float.to_le_bytes());
        }
        res
    }    
}

impl UniformType for bool {
    fn get_quad_type(&self) -> miniquad::UniformType {
        miniquad::UniformType::Int1
    }

    fn encode_u8(&self) -> Vec<u8> {
        vec![unsafe { std::mem::transmute(*self) }]
    }    
}

#[derive(Clone)]
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

    pub fn get_quad_type(&self) -> miniquad::UniformType {
        self.value.get_quad_type()
    }

    pub fn encode_u8(&self) -> Vec<u8> {
        self.value.encode_u8()
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