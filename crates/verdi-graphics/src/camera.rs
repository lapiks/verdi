use std::ops::Deref;

use mlua::{UserData, UserDataFields};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};
use verdi_math::{Mat4, prelude::TransformHandle};

pub type CameraId = ResourceId;

#[derive(Clone)] // TODO: is it really clonable?
pub struct Camera {
    pub transform: TransformHandle,
    pub id: CameraId,
}

impl Resource for Camera {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Camera {
    pub fn new(transform: TransformHandle) -> Self {
        Self {
            transform,
            id: CameraId::null(),
        }
    }

    // pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4 {
    //     let f = {
    //         let f = direction;
    //         let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
    //         let len = len.sqrt();
    //         [f[0] / len, f[1] / len, f[2] / len]
    //     };
    
    //     let s = [up[1] * f[2] - up[2] * f[1],
    //              up[2] * f[0] - up[0] * f[2],
    //              up[0] * f[1] - up[1] * f[0]];
    
    //     let s_norm = {
    //         let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    //         let len = len.sqrt();
    //         [s[0] / len, s[1] / len, s[2] / len]
    //     };
    
    //     let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
    //              f[2] * s_norm[0] - f[0] * s_norm[2],
    //              f[0] * s_norm[1] - f[1] * s_norm[0]];
    
    //     let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
    //              -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
    //              -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        

    //     Mat4::from_cols_array_2d(
    //         &[
    //             [s_norm[0], u[0], f[0], 0.0],
    //             [s_norm[1], u[1], f[1], 0.0],
    //             [s_norm[2], u[2], f[2], 0.0],
    //             [p[0], p[1], p[2], 1.0],
    //         ]
    //     )
    // }

    pub fn orthographic_matrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32
    ) -> Mat4 {
        Mat4::from_cols_array_2d(
            &[
                [2.0 / (right - left)            , 0.0                             , 0.0                         , 0.0],
                [0.0                             , 2.0 / (top - bottom)            , 0.0                         , 0.0],
                [0.0                             , 0.0                             , -2.0 / (far - near)        , 0.0],
                [-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near) / (far - near),   1.0],
            ]
        )
    }

    pub fn perspective_matrix(width: u32, height: u32) -> Mat4 {
        let aspect_ratio = height as f32 / width as f32;
    
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan();
        
        Mat4::from_cols_array_2d(
            &[
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        )
    }
}

#[derive(Clone)]
pub struct CameraHandle(Handle<Camera>);

impl Deref for CameraHandle {
    type Target = Handle<Camera>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CameraHandle {
    pub fn new(assets: Assets, id: CameraId) -> Self{
        CameraHandle(assets.new_handle(id))
    }
}

impl UserData for CameraHandle {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("transform", |_, this| {
            Ok({
                this.get_datas()
                    .get::<Camera>(this.get_id())
                    .expect("Camera not found")
                    .transform.clone()
            })
        });
    }
    
    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

    // }
}