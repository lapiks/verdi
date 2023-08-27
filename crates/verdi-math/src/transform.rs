use std::ops::{Deref, DerefMut};

use mlua::{UserData, UserDataMethods};
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{Vec3, Quat, Mat4, types::LuaVec3};

pub type TransformId = ResourceId;

#[derive(PartialEq, Clone)]
pub struct Transform {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Resource for Transform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Transform {
    pub const IDENTITY: Self = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new() -> Self {
        Transform::IDENTITY
    }

    pub fn from_matrix(matrix: Mat4) -> Self {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn reset(&mut self) {
        *self = Transform::IDENTITY
    }
    
    pub fn translate(&mut self, vec: Vec3) {
        self.translation += vec;
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        self.rotation *= Quat::from_axis_angle(axis, angle);
    }

    pub fn scale(&mut self, factor: Vec3) {
        self.scale *= factor;
    }

    pub fn set_position(&mut self, vec: Vec3) {
        self.translation = vec;
    }

    pub fn get_position(&self) -> Vec3 {
        self.translation
    }

    pub fn set_rotation(&mut self, angle: f32, axis: Vec3) {
        self.rotation = Quat::from_axis_angle(axis, angle);
    }

    pub fn set_scale(&mut self, factor: Vec3) {
        self.scale = factor;
    }

    pub fn apply(&mut self, other: &Transform) {
        other.transform_point(self.translation);
        self.rotation *= other.rotation;
        self.scale *= other.scale;
    }

    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        point *= self.scale;
        point = self.rotation * point;
        point += self.translation;
        
        point
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::NEG_X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Y
    }
    
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::IDENTITY
    }
}

#[derive(Clone)]
pub struct TransformHandle(Handle);

impl Deref for TransformHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TransformHandle {
    fn deref_mut(&mut self) -> &mut Handle {
        &mut self.0
    }
}

impl TransformHandle {
    pub fn new(assets: Assets, id: TransformId) -> Self {
        TransformHandle(assets.new_handle(id))

    }
}


impl UserData for TransformHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("reset", |_, this, ()| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.reset();
                }
            })
        });

        methods.add_method_mut("translate", |_, this, v: LuaVec3| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.translate(Vec3::from(v));
                }
            })
        });

        methods.add_method_mut("rotate", |_, this, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.rotate(angle, Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("scale", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.scale(Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("setPosition", |_, this, v: LuaVec3| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.set_position(Vec3::from(v));
                }
            })
        });

        methods.add_method("getPosition", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.get_position());
                }
            })
        });

        methods.add_method_mut("setRotation", |_, this, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.set_rotation(angle, Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("setScale", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    transform.set_scale(Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("apply", |_, this, other: TransformHandle| {
            Ok({
                let transform_id = this.get_id();
                if let Some(transform) = this.get_datas_mut().get_mut::<Transform>(transform_id) {
                    if let Some(other_transform) = other.get_datas().get(other.get_id()) {
                        transform.apply(&other_transform);
                    }
                }
            })
        });

        methods.add_method("transformPoint", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(
                        transform.transform_point(
                            Vec3::new(x, y, z)
                        )
                    );
                }
            })
        });

        methods.add_method("right", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.right());
                }
            })
        });

        methods.add_method("left", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.left());
                }
            })
        });

        methods.add_method("up", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.up());
                }
            })
        });

        methods.add_method("down", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.down());
                }
            })
        });

        methods.add_method("forward", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.forward());
                }
            })
        });

        methods.add_method("backward", |_, this, ()| {
            Ok({
                if let Some(transform) = this.get_datas().get::<Transform>(this.get_id()) {
                    LuaVec3(transform.backward());
                }
            })
        });
    }
}