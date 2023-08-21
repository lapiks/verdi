use glam::Mat4;
use verdi_database::Assets;

use crate::transform::{TransformHandle, Transform};

pub struct Math {
    pub(crate) assets: Assets,
} 

impl Math {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),
        }
    }

    pub fn new_transform(&mut self) -> TransformHandle {
        TransformHandle::new(
            self.assets.clone(),
            self.assets.add(Box::new(Transform::new()))
        )
    }

    pub fn new_transform_from_matrix(&mut self, mat: Mat4) -> TransformHandle {
        TransformHandle::new(
            self.assets.clone(),
            self.assets.add(Box::new(Transform::from_matrix(mat)))
        )
    }
}