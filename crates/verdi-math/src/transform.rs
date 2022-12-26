use rlua::UserData;

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
}

impl Default for Transform {
    fn default() -> Self {
        Transform::IDENTITY
    }
}

#[derive(Clone, Copy)]
pub struct TransformRef {
    //pub id: TransformId,
}

impl TransformRef {
    pub fn new() -> Self {
        Self { }
    }
}

impl UserData for TransformRef {}