use mlua::{UserData, UserDataMethods};

use crate::{Vec3, Quat, Mat4};

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

    pub fn translate(&mut self, vec: Vec3) {
        self.translation += vec;
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        self.rotation *= Quat::from_axis_angle(axis, angle);
    }

    pub fn scale(&mut self, factor: Vec3) {
        self.scale *= factor;
    }

    pub fn apply(&mut self, other: &Transform) {
        other.transform_point(self.translation);
        self.rotation *= other.rotation;
        self.scale *= other.scale;
    }

    pub fn transform_point(&self, mut point: Vec3) -> (f32, f32, f32) {
        point *= self.scale;
        point = self.rotation * point;
        point += self.translation;
        
        (point.x, point.y, point.z)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::IDENTITY
    }
}

impl UserData for Transform {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("translate", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(transform.translate(Vec3::new(x, y, z)))
        });

        methods.add_method_mut("rotate", |_, transform, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok(transform.rotate(angle, Vec3::new(x, y, z)))
        });

        methods.add_method_mut("scale", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(transform.scale(Vec3::new(x, y, z)))
        });

        methods.add_method_mut("apply", |_, transform, other: Transform| {
            Ok(transform.apply(&other))
        });

        methods.add_method("transformPoint", |_, transform, (x, y, z): (f32, f32, f32)| {
            Ok(transform.transform_point(Vec3::new(x, y, z)))
        });
    }
}