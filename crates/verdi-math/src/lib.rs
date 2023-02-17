#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        bind_math::BindMath,
        transform::*,
        // glam types
        Vec2, Vec3, Vec4,
        IVec2, IVec3, IVec4,
        Mat2, Mat3, Mat4,
        Quat,
        // ...   
        // verdi types
        types::{
            LuaVec2, LuaVec3, LuaVec4,
            LuaIVec2, LuaIVec3, LuaIVec4,
            LuaMat2, LuaMat3, LuaMat4,
            LuaQuat,
        }
    };
}

pub use glam::*;

mod transform;
mod bind_math;
mod types;
