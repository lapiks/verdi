use std::{cell::RefCell, rc::Rc};

use mlua::{UserData, UserDataMethods};
use slotmap::{new_key_type, Key};
use verdi_math::{Mat4, prelude::Transform, Vec3};

use crate::database::Database;

new_key_type! {
    pub struct CameraId;
}

pub struct Camera {
    pub transform: Transform,
    pub id: CameraId,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            id: CameraId::null(),
        }
    }

    pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4 {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        

        Mat4::from_cols_array_2d(
            &[
                [s_norm[0], u[0], f[0], 0.0],
                [s_norm[1], u[1], f[1], 0.0],
                [s_norm[2], u[2], f[2], 0.0],
                [p[0], p[1], p[2], 1.0],
            ]
        )
    }

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
pub struct CameraHandle {
    pub database: Rc<RefCell<Database>>,
    pub id: CameraId,
}

impl UserData for CameraHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("transform", |_, camera, ()| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform
                }
                else {
                    Transform::new()
                }
            })
        });

        methods.add_method_mut("reset", |_, camera, ()| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.reset();
                }
            })
        });

        methods.add_method_mut("translate", |_, camera, (x, y, z): (f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.translate(Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("rotate", |_, camera, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.rotate(angle, Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("scale", |_, camera, (x, y, z): (f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.scale(Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("setPosition", |_, camera, (x, y, z): (f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.set_position(Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("setRotation", |_, camera, (angle, x, y, z): (f32, f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.set_rotation(angle, Vec3::new(x, y, z));
                }
            })
        });

        methods.add_method_mut("setScale", |_, camera, (x, y, z): (f32, f32, f32)| {
            Ok({
                if let Some(camera) = camera.database.borrow_mut().assets.get_camera_mut(camera.id) {
                    camera.transform.set_scale(Vec3::new(x, y, z));
                }
            })
        });
    }
}