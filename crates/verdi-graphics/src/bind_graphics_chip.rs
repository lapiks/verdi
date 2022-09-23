use std::sync::Mutex;
use rlua::{Lua, Result};

use verdi_math::prelude::*;

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType, image::ImageRef, scene::Scene};

pub struct BindGraphicsChip;

impl<'lua> BindGraphicsChip {
    fn begin_object(gpu: &Mutex<GraphicsChip>, primitive_type: &String) {
        let mut enum_val = PrimitiveType::Triangles;
        if primitive_type == "triangles" { enum_val = PrimitiveType::Triangles; }
        else if primitive_type == "points" { enum_val = PrimitiveType::Points; }
        else if primitive_type == "lines" { enum_val = PrimitiveType::Lines; }

        gpu.lock().unwrap().begin(enum_val);
    }

    fn end_object(gpu: &Mutex<GraphicsChip>) {
        gpu.lock().unwrap().end();
    }

    fn vertex(gpu: &Mutex<GraphicsChip>, x: f32, y: f32, z: f32) {
        gpu.lock().unwrap().vertex(Vec3::new(x, y, z));
    }

    fn normal(gpu: &Mutex<GraphicsChip>, x: f32, y: f32, z: f32) {
        gpu.lock().unwrap().normal(Vec3::new(x, y, z));
    }

    fn tex_coord(gpu: &Mutex<GraphicsChip>, u: f32, v: f32) {
        gpu.lock().unwrap().tex_coord(Vec2::new(u, v));
    }

    fn color(gpu: &Mutex<GraphicsChip>, r: f32, g: f32, b: f32, a: f32) {
        gpu.lock().unwrap().color(Vec4::new(r, g, b, a));
    }

    fn bind_texture(gpu: &Mutex<GraphicsChip>, image: ImageRef) {
        gpu.lock().unwrap().bind_texture(image);
    }

    fn new_image(gpu: &Mutex<GraphicsChip>, path: &String) -> ImageRef {
        gpu.lock().unwrap().new_image(path).unwrap()
    }

    fn new_scene(gpu: &Mutex<GraphicsChip>, path: &String) -> Scene{
        gpu.lock().unwrap().new_scene(path).unwrap()
    }

    fn draw(gpu: &Mutex<GraphicsChip>, scene: &Scene) {
        gpu.lock().unwrap().draw(scene)
    }

    pub fn bind(lua: &Lua, gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create graphics module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                let func = lua_ctx.create_function_mut(|_, primitive_type: String| Ok(BindGraphicsChip::begin_object(gpu, &primitive_type)))?;
                module_table.set("beginObject", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, ()| Ok(BindGraphicsChip::end_object(gpu)))?;
                module_table.set("endObject", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, (x, y ,z): (f32 , f32, f32)| Ok(BindGraphicsChip::vertex(gpu, x, y, z)))?;
                module_table.set("vertex", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, (x, y ,z): (f32 , f32, f32)| Ok(BindGraphicsChip::normal(gpu, x, y, z)))?;
                module_table.set("normal", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, (u, v): (f32 , f32)| Ok(BindGraphicsChip::tex_coord(gpu, u, v)))?;
                module_table.set("tex_coord", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, (r, g, b, a): (f32 , f32, f32, f32)| Ok(BindGraphicsChip::color(gpu, r, g, b, a)))?;
                module_table.set("color", func)?;
            }
            {
                let func = lua_ctx.create_function(|_, path: String| Ok(BindGraphicsChip::new_image(gpu, &path)))?;
                module_table.set("newImage", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, image: ImageRef| Ok(BindGraphicsChip::bind_texture(gpu, image)))?;
                module_table.set("bindTexture", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, path: String| Ok(BindGraphicsChip::new_scene(gpu, &path)))?;
                module_table.set("newScene", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, scene: Scene| Ok(BindGraphicsChip::draw(gpu, &scene)))?;
                module_table.set("draw", func)?;
            }

            // add table to globals
            globals.set("graphics", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}