pub use glam::*;
use rlua::UserData;

// wrapping glam types in our own types to be able to implement external rlua traits on them
pub struct Vec2(glam::Vec2);
impl UserData for Vec2 {}

pub struct Vec3(glam::Vec3);
impl UserData for Vec3 {}

pub struct Vec4(glam::Vec4);
impl UserData for Vec4 {}

pub struct IVec2(glam::IVec2);
impl UserData for IVec2 {}

pub struct IVec3(glam::IVec3);
impl UserData for IVec3 {}

pub struct IVec4(glam::IVec4);
impl UserData for IVec4 {}

pub struct Mat2(glam::Mat2);
impl UserData for Mat2 {}

pub struct Mat3(glam::Mat3);
impl UserData for Mat3 {}

pub struct Mat4(glam::Mat4);
impl UserData for Mat4 {}

pub struct Quat(glam::Quat);
impl UserData for Quat {}

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        Vec2, Vec3, Vec4,
        IVec2, IVec3, IVec4,
        Mat2, Mat3, Mat4,
        Quat,
        //...   
    };
}