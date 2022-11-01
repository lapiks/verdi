use std::sync::{Mutex, Arc};
use rlua::{Lua, Result};

use verdi_math::prelude::*;

use crate::{
    prelude::GraphicsChip, 
    graphics_chip::PrimitiveType, 
    image::ImageRef, 
    scene::SceneRef
};

pub struct BindGraphicsChip;

impl<'lua> BindGraphicsChip {
    fn begin_object(gpu: Arc<Mutex<GraphicsChip>>, primitive_type: &String) {
        let enum_val = PrimitiveType::from(primitive_type.clone());
        gpu.lock().unwrap().begin(enum_val);
    }

    fn end_object(gpu: Arc<Mutex<GraphicsChip>>) {
        gpu.lock().unwrap().end();
    }

    fn vertex(gpu: Arc<Mutex<GraphicsChip>>, coords: &Vec3) {
        gpu.lock().unwrap().vertex(coords);
    }

    fn normal(gpu: Arc<Mutex<GraphicsChip>>, coords: &Vec3) {
        gpu.lock().unwrap().normal(coords);
    }

    fn tex_coord(gpu: Arc<Mutex<GraphicsChip>>, coords: &Vec2) {
        gpu.lock().unwrap().tex_coord(coords);
    }

    fn color(gpu: Arc<Mutex<GraphicsChip>>, color: &Vec4) {
        gpu.lock().unwrap().color(color);
    }

    fn bind_texture(gpu: Arc<Mutex<GraphicsChip>>, image: ImageRef) {
        gpu.lock().unwrap().bind_texture(image);
    }

    fn new_image(gpu: Arc<Mutex<GraphicsChip>>, path: &String) -> ImageRef {
        let image_id = gpu.lock().unwrap().new_image(path).unwrap();
        ImageRef::new(image_id)
    }

    fn new_scene(gpu: Arc<Mutex<GraphicsChip>>, path: &String) -> SceneRef {
        let mut gpu_guard = gpu.lock().unwrap();
        let scene_id = gpu_guard.new_scene(path).unwrap();
        SceneRef::new(gpu.clone(), scene_id)
    }
    
    fn set_clear_color(gpu: Arc<Mutex<GraphicsChip>>, color: &Vec4) {
        gpu.lock().unwrap().set_clear_color(color);
    }

    fn translate(gpu: Arc<Mutex<GraphicsChip>>, v: &Vec3) {
        gpu.lock().unwrap().translate(v);
    }

    fn rotate(gpu: Arc<Mutex<GraphicsChip>>, angle: f32, axis: &Vec3) {
        gpu.lock().unwrap().rotate(angle, axis);
    }

    fn set_fog_start(gpu: Arc<Mutex<GraphicsChip>>, distance: f32) {
        gpu.lock().unwrap().set_fog_start(distance);
    }

    fn set_fog_end(gpu: Arc<Mutex<GraphicsChip>>, distance: f32) {
        gpu.lock().unwrap().set_fog_end(distance);
    }

    fn draw_line(gpu: Arc<Mutex<GraphicsChip>>, p1: &Vec2, p2: &Vec2) {
        gpu.lock().unwrap().draw_line(p1, p2);
    }

    pub fn bind(lua: &Lua, gpu: Arc<Mutex<GraphicsChip>>) -> Result<()> {
        lua.context(move |lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create graphics module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, primitive_type: String| Ok(BindGraphicsChip::begin_object(gpu.clone(), &primitive_type)))?;
                module_table.set("beginObject", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, ()| Ok(BindGraphicsChip::end_object(gpu.clone())))?;
                module_table.set("endObject", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (x, y ,z): (f32 , f32, f32)| Ok(
                        BindGraphicsChip::vertex(
                            gpu.clone(), 
                            &Vec3::new(x, y, z)
                        )
                    )
                )?;
                module_table.set("vertex", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (x, y ,z): (f32 , f32, f32)| Ok(
                        BindGraphicsChip::normal(
                            gpu.clone(), 
                            &Vec3::new(x, y, z)
                        )
                    )
                )?;
                module_table.set("normal", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (u, v): (f32 , f32)| Ok(
                        BindGraphicsChip::tex_coord(
                            gpu.clone(), 
                            &Vec2::new(u, v)
                        )
                    )
                )?;
                module_table.set("tex_coord", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (r, g, b, a): (f32 , f32, f32, f32)| Ok(
                        BindGraphicsChip::color(
                            gpu.clone(), 
                            &Vec4::new(r, g, b, a)
                        )  
                    )
                )?;
                module_table.set("color", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function(move |_, path: String| Ok(BindGraphicsChip::new_image(gpu.clone(), &path)))?;
                module_table.set("newImage", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, image: ImageRef| Ok(BindGraphicsChip::bind_texture(gpu.clone(), image)))?;
                module_table.set("bindTexture", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, path: String| Ok(BindGraphicsChip::new_scene(gpu.clone(), &path)))?;
                module_table.set("newScene", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (r, g, b, a): (f32, f32, f32, f32)| Ok(
                        BindGraphicsChip::set_clear_color(
                            gpu.clone(), 
                            &Vec4::new(r, g, b, a)
                        )
                    )
                )?;
                module_table.set("setClearColor", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, distance: f32| Ok(BindGraphicsChip::set_fog_start(gpu.clone(), distance)))?;
                module_table.set("setFogStart", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, distance: f32| Ok(BindGraphicsChip::set_fog_end(gpu.clone(), distance)))?;
                module_table.set("setFogEnd", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (x1, y1, x2, y2): (f32, f32, f32, f32)| Ok(
                        BindGraphicsChip::draw_line(
                            gpu.clone(), 
                            &Vec2::new(x1, y1), 
                            &Vec2::new(x2, y2)
                        )
                    )
                )?;
                module_table.set("drawLine", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (x, y, z): (f32, f32, f32)| Ok(
                        BindGraphicsChip::translate(
                            gpu.clone(),
                            &Vec3::new(x, y, z)
                        )
                    )
                )?;
                module_table.set("translate", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(
                    move |_, (angle, x, y, z): (f32, f32, f32, f32)| Ok(
                        BindGraphicsChip::rotate(
                            gpu.clone(),
                            angle,
                            &Vec3::new(x, y, z)
                        )
                    )
                )?;
                module_table.set("rotate", func)?;
            }

            // add table to globals
            globals.set("graphics", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}