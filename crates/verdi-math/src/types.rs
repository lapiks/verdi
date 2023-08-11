use std::ops::{Deref, DerefMut};

use glam::{
    Vec2, Vec3, Vec4, 
    IVec2, IVec3, IVec4, 
    Mat2, Mat3, Mat4, 
    Quat
};

use mlua::{UserData, UserDataFields, UserDataMethods, MetaMethod};

// wrapping glam types in our own types to be able to implement external mlua UserData trait on them

// Vec2
#[derive(Clone, Copy, PartialEq)]
pub struct LuaVec2(pub Vec2);

impl Deref for LuaVec2 {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaVec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec2> for LuaVec2 {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl UserData for LuaVec2 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaVec2, LuaVec2)| {
            Ok(LuaVec2(v1.0 + v2.0))
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (v1, scalar): (LuaVec2, f32)| {
            Ok(LuaVec2(v1.0 * scalar))
        });
    }
}

// Vec3
#[derive(Clone, Copy, PartialEq)]
pub struct LuaVec3(pub Vec3);

impl Deref for LuaVec3 {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaVec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for LuaVec3 {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl UserData for LuaVec3 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("z", |_, this| Ok(this.z));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3(v1.0 + v2.0))
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (v1, scalar): (LuaVec3, f32)| {
            Ok(LuaVec3(v1.0 * scalar))
        });
    }
}

// Vec4
#[derive(Clone, Copy, PartialEq)]
pub struct LuaVec4(pub Vec4);

impl Deref for LuaVec4 {
    type Target = Vec4;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaVec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec4> for LuaVec4 {
    fn from(v: Vec4) -> Self {
        Self(v)
    }
}

impl UserData for LuaVec4 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("z", |_, this| Ok(this.z));
        fields.add_field_method_get("w", |_, this| Ok(this.w));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaVec4, LuaVec4)| {
            Ok(LuaVec4(v1.0 + v2.0))
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (v1, scalar): (LuaVec4, f32)| {
            Ok(LuaVec4(v1.0 * scalar))
        });
    }
}

// IVec2
#[derive(Clone, Copy, PartialEq)]
pub struct LuaIVec2(pub IVec2);

impl Deref for LuaIVec2 {
    type Target = IVec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaIVec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IVec2> for LuaIVec2 {
    fn from(v: IVec2) -> Self {
        Self(v)
    }
}

impl UserData for LuaIVec2 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaIVec2, LuaIVec2)| {
            Ok(LuaIVec2(v1.0 + v2.0))
        });
    }
}

// IVec3
#[derive(Clone, Copy, PartialEq)]
pub struct LuaIVec3(pub IVec3);

impl Deref for LuaIVec3 {
    type Target = IVec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaIVec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IVec3> for LuaIVec3 {
    fn from(v: IVec3) -> Self {
        Self(v)
    }
}

impl UserData for LuaIVec3 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("z", |_, this| Ok(this.z));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaIVec3, LuaIVec3)| {
            Ok(LuaIVec3(v1.0 + v2.0))
        });
    }
}

// IVec4
#[derive(Clone, Copy, PartialEq)]
pub struct LuaIVec4(pub IVec4);

impl Deref for LuaIVec4 {
    type Target = IVec4;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaIVec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IVec4> for LuaIVec4 {
    fn from(v: IVec4) -> Self {
        Self(v)
    }
}

impl UserData for LuaIVec4 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("z", |_, this| Ok(this.z));
        fields.add_field_method_get("w", |_, this| Ok(this.w));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_, (v1, v2): (LuaIVec4, LuaIVec4)| {
            Ok(LuaIVec4(v1.0 + v2.0))
        });
    }
}

// Mat2
#[derive(Clone, Copy, PartialEq)]
pub struct LuaMat2(pub Mat2);

impl Deref for LuaMat2 {
    type Target = Mat2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMat2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Mat2> for LuaMat2 {
    fn from(v: Mat2) -> Self {
        Self(v)
    }
}

impl UserData for LuaMat2 {}

// Mat3
#[derive(Clone, Copy, PartialEq)]
pub struct LuaMat3(pub Mat3);

impl Deref for LuaMat3 {
    type Target = Mat3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMat3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Mat3> for LuaMat3 {
    fn from(v: Mat3) -> Self {
        Self(v)
    }
}

impl UserData for LuaMat3 {}

// Mat4
#[derive(Clone, Copy, PartialEq)]
pub struct LuaMat4(pub Mat4);

impl Deref for LuaMat4 {
    type Target = Mat4;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMat4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Mat4> for LuaMat4 {
    fn from(v: Mat4) -> Self {
        Self(v)
    }
}

impl UserData for LuaMat4 {}

// Quat
#[derive(Clone, Copy, PartialEq)]
pub struct LuaQuat(pub Quat);

impl Deref for LuaQuat {
    type Target = Quat;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaQuat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Quat> for LuaQuat {
    fn from(v: Quat) -> Self {
        Self(v)
    }
}

impl UserData for LuaQuat {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("z", |_, this| Ok(this.z));
        fields.add_field_method_get("w", |_, this| Ok(this.w));
    }
}


impl From<LuaVec2> for Vec2 {
    fn from(v: LuaVec2) -> Self {
        v.0
    }
}

impl From<LuaVec3> for Vec3 {
    fn from(v: LuaVec3) -> Self {
        v.0
    }
}

impl From<LuaVec4> for Vec4 {
    fn from(v: LuaVec4) -> Self {
        v.0
    }
}

impl From<LuaIVec2> for IVec2 {
    fn from(v: LuaIVec2) -> Self {
        v.0
    }
}

impl From<LuaIVec3> for IVec3 {
    fn from(v: LuaIVec3) -> Self {
        v.0
    }
}

impl From<LuaIVec4> for IVec4 {
    fn from(v: LuaIVec4) -> Self {
        v.0
    }
}

impl From<LuaMat2> for Mat2 {
    fn from(v: LuaMat2) -> Self {
        v.0
    }
}

impl From<LuaMat3> for Mat3 {
    fn from(v: LuaMat3) -> Self {
        v.0
    }
}

impl From<LuaMat4> for Mat4 {
    fn from(v: LuaMat4) -> Self {
        v.0
    }
}

impl From<LuaQuat> for Quat {
    fn from(v: LuaQuat) -> Self {
        v.0
    }
}