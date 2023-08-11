use mlua::{UserData, UserDataMethods};

use crate::{Vec3, Quat, Mat4, types::LuaVec3};

#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
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

impl UserData for Transform {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("reset", |_, transform, ()| {
            Ok(transform.reset())
        });

        // methods.add_method_mut("translate", |_, transform, (x, y, z): (f32, f32, f32)| {
        //     Ok(transform.translate(Vec3::new(x, y, z)))
        // });

        methods.add_method_mut("translate", |_, transform, v: LuaVec3| {
            Ok(transform.translate(Vec3::from(v)))
        });

        methods.add_method_mut("rotate", |_, transform, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok(transform.rotate(angle, Vec3::new(x, y, z)))
        });

        methods.add_method_mut("scale", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(transform.scale(Vec3::new(x, y, z)))
        });

        // methods.add_method_mut("setPosition", |_, transform, (x, y, z): (f32, f32, f32)| {
        //     Ok(transform.set_position(Vec3::new(x, y, z)))
        // });

        methods.add_method_mut("setPosition", |_, transform, v: LuaVec3| {
            Ok(transform.set_position(Vec3::from(v)))
        });

        methods.add_method("getPosition", |_, transform, ()| {
            Ok(LuaVec3(transform.get_position()))
        });

        methods.add_method_mut("setRotation", |_, transform, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok(transform.set_rotation(angle, Vec3::new(x, y, z)))
        });

        methods.add_method_mut("setScale", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(transform.set_scale(Vec3::new(x, y, z)))
        });

        methods.add_method_mut("apply", |_, transform, other: Transform| {
            Ok(transform.apply(&other))
        });

        methods.add_method("transformPoint", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(
                LuaVec3(
                    transform.transform_point(
                        Vec3::new(x, y, z)
                    )
                )
            )
        });

        methods.add_method("right", |_, transform, ()| {
            Ok(LuaVec3(transform.right()))
        });

        methods.add_method("left", |_, transform, ()| {
            Ok(LuaVec3(transform.left()))
        });

        methods.add_method("up", |_, transform, ()| {
            Ok(LuaVec3(transform.up()))
        });

        methods.add_method("down", |_, transform, ()| {
            Ok(LuaVec3(transform.down()))
        });

        methods.add_method("forward", |_, transform, ()| {
            Ok(LuaVec3(transform.forward()))
        });

        methods.add_method("backward", |_, transform, ()| {
            Ok(LuaVec3(transform.backward()))
        });
    }
}