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

    fn vertex(gpu: Arc<Mutex<GraphicsChip>>, x: f32, y: f32, z: f32) {
        gpu.lock().unwrap().vertex(Vec3::new(x, y, z));
    }

    fn normal(gpu: Arc<Mutex<GraphicsChip>>, x: f32, y: f32, z: f32) {
        gpu.lock().unwrap().normal(Vec3::new(x, y, z));
    }

    fn tex_coord(gpu: Arc<Mutex<GraphicsChip>>, u: f32, v: f32) {
        gpu.lock().unwrap().tex_coord(Vec2::new(u, v));
    }

    fn color(gpu: Arc<Mutex<GraphicsChip>>, r: f32, g: f32, b: f32, a: f32) {
        gpu.lock().unwrap().color(Vec4::new(r, g, b, a));
    }

    fn bind_texture(gpu: Arc<Mutex<GraphicsChip>>, image: ImageRef) {
        gpu.lock().unwrap().bind_texture(image);
    }

    fn new_image(gpu: Arc<Mutex<GraphicsChip>>, path: &String) -> ImageRef {
        gpu.lock().unwrap().new_image(path).unwrap()
    }

    // fn draw(gpu: &Mutex<GraphicsChip>, scene: &Scene) {
    //     gpu.lock().unwrap().draw(scene)
    // }

    fn new_scene(gpu: Arc<Mutex<GraphicsChip>>, path: &String) -> SceneRef {
        let mut gpu_guard = gpu.lock().unwrap();
        let scene = gpu_guard.new_scene(path).unwrap();
        SceneRef::new(gpu.clone(), scene.id)
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
                let func = lua_ctx.create_function_mut(move |_, (x, y ,z): (f32 , f32, f32)| Ok(BindGraphicsChip::vertex(gpu.clone(), x, y, z)))?;
                module_table.set("vertex", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, (x, y ,z): (f32 , f32, f32)| Ok(BindGraphicsChip::normal(gpu.clone(), x, y, z)))?;
                module_table.set("normal", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, (u, v): (f32 , f32)| Ok(BindGraphicsChip::tex_coord(gpu.clone(), u, v)))?;
                module_table.set("tex_coord", func)?;
            }
            {
                let gpu = gpu.clone();
                let func = lua_ctx.create_function_mut(move |_, (r, g, b, a): (f32 , f32, f32, f32)| Ok(BindGraphicsChip::color(gpu.clone(), r, g, b, a)))?;
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
            // {
            //     let func = lua_ctx.create_function_mut(|_, scene: Scene| Ok(BindGraphicsChip::draw(gpu, &scene)))?;
            //     module_table.set("draw", func)?;
            // }

            // add table to globals
            globals.set("graphics", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}