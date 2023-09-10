use std::{rc::Rc, cell::RefCell};
use mlua::{Lua, Result};

use verdi_math::prelude::*;

use crate::{
    prelude::GraphicsChip, 
    image::ImageHandle, 
    model::ModelHandle, 
    mesh::{MeshHandle, PrimitiveType}, 
    material::MaterialHandle, 
    camera::CameraHandle, 
    sprite::SpriteHandle, 
    uniform::UniformHandle
};

pub struct BindGraphicsChip;

impl<'lua> BindGraphicsChip {
    fn begin_object(gpu: &mut GraphicsChip, primitive_type: &String) {
        let enum_val = PrimitiveType::from(primitive_type.clone());
        gpu.begin(enum_val);
    }

    fn end_object(gpu: &mut GraphicsChip) {
        gpu.end();
    }

    fn vertex(gpu: &mut GraphicsChip, coords: &Vec3) {
        gpu.vertex(coords);
    }

    fn normal(gpu: &mut GraphicsChip, coords: &Vec3) {
        gpu.normal(coords);
    }

    fn tex_coord(gpu: &mut GraphicsChip, coords: &Vec2) {
        gpu.tex_coord(coords);
    }

    fn color(gpu: &mut GraphicsChip, color: &Vec4) {
        gpu.color(color);
    }

    fn bind_texture(gpu: &mut GraphicsChip, image: ImageHandle) {
        gpu.bind_texture(image);
    }

    // object construction
    fn new_image(gpu: Rc<RefCell<GraphicsChip>>, path: &String) -> ImageHandle {
        let image_id = gpu.borrow_mut().new_image(path).unwrap();
        ImageHandle::new(gpu.borrow().assets.clone(), image_id)
    }

    fn new_model(gpu: Rc<RefCell<GraphicsChip>>, path: &String) -> ModelHandle {
        let model_id = gpu.borrow_mut().new_model(path).unwrap();
        ModelHandle::new(gpu.borrow().assets.clone(), model_id)
    }
    
    fn new_mesh(gpu: Rc<RefCell<GraphicsChip>>) -> MeshHandle {
        let mesh_id = gpu.borrow_mut().new_mesh().unwrap();
        MeshHandle::new(gpu.borrow().assets.clone(), mesh_id)
    }

    // fn new_sprite(gpu: Rc<RefCell<GraphicsChip>>, image: ImageHandle) -> SpriteHandle {
    //     let sprite_id = gpu.borrow_mut().new_sprite(image);
    //     SpriteHandle { 
    //         assets: gpu.borrow().assets.clone(), 
    //         id: sprite_id,
    //     }
    // }

    fn new_material(gpu: Rc<RefCell<GraphicsChip>>) -> MaterialHandle {
        let mat_id = gpu.borrow_mut().new_gouraud_material();
        MaterialHandle::new(
            gpu.borrow().assets.clone(), 
            mat_id
        )
    }

    fn new_camera(gpu: Rc<RefCell<GraphicsChip>>, transform: TransformHandle) -> CameraHandle {
        gpu.borrow_mut().new_camera(transform)
    }

    // fn new_uniform(gpu: Rc<RefCell<GraphicsChip>>, value: f32) -> UniformHandle {
    //     let uniform_id = gpu.borrow_mut().new_uniform(value);
    //     UniformHandle::new(
    //         gpu.borrow().assets.clone(), 
    //         uniform_id
    //     )
    // }

    fn set_clear_color(gpu: &mut GraphicsChip, color: &Vec4) {
        gpu.set_clear_color(color);
    }

    fn draw_line(gpu: &mut GraphicsChip, p1: &Vec2, p2: &Vec2) {
        gpu.draw_line(p1, p2);
    }

    pub fn bind(lua: &Lua, gpu: Rc<RefCell<GraphicsChip>>) -> Result<()> {
        let globals = lua.globals();

        // create graphics module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, primitive_type: String| Ok(BindGraphicsChip::begin_object(&mut gpu.borrow_mut(), &primitive_type)))?;
            module_table.set("beginObject", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, ()| Ok(BindGraphicsChip::end_object(&mut gpu.borrow_mut())))?;
            module_table.set("endObject", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (x, y ,z): (f32 , f32, f32)| Ok(
                    BindGraphicsChip::vertex(
                        &mut gpu.borrow_mut(), 
                        &Vec3::new(x, y, z)
                    )
                )
            )?;
            module_table.set("vertex", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (x, y ,z): (f32 , f32, f32)| Ok(
                    BindGraphicsChip::normal(
                        &mut gpu.borrow_mut(), 
                        &Vec3::new(x, y, z)
                    )
                )
            )?;
            module_table.set("normal", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (u, v): (f32 , f32)| Ok(
                    BindGraphicsChip::tex_coord(
                        &mut gpu.borrow_mut(), 
                        &Vec2::new(u, v)
                    )
                )
            )?;
            module_table.set("tex_coord", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (r, g, b, a): (f32 , f32, f32, f32)| Ok(
                    BindGraphicsChip::color(
                        &mut gpu.borrow_mut(), 
                        &Vec4::new(r, g, b, a)
                    )  
                )
            )?;
            module_table.set("color", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, image: ImageHandle| Ok(BindGraphicsChip::bind_texture(&mut gpu.borrow_mut(), image)))?;
            module_table.set("bindTexture", func)?;
        }
        // New objects
        {
            let gpu = gpu.clone();
            let func = lua.create_function(move |_, path: String| Ok(BindGraphicsChip::new_image(gpu.clone(), &path)))?;
            module_table.set("newImage", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, path: String| Ok(BindGraphicsChip::new_model(gpu.clone(), &path)))?;
            module_table.set("newModel", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, ()| Ok(BindGraphicsChip::new_mesh(gpu.clone())))?;
            module_table.set("newMesh", func)?;
        }
        // {
        //     let gpu = gpu.clone();
        //     let func = lua.create_function_mut(move |_, image: ImageHandle| Ok(BindGraphicsChip::new_sprite(gpu.clone(), image)))?;
        //     module_table.set("newSprite", func)?;
        // }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, ()| Ok(BindGraphicsChip::new_material(gpu.clone())))?;
            module_table.set("newMaterial", func)?;
        }
        // {
        //     let gpu = gpu.clone();
        //     let func = lua.create_function_mut(move |_, value: f32| Ok(BindGraphicsChip::new_uniform(gpu.clone(), value)))?;
        //     module_table.set("newUniform", func)?;
        // }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(move |_, transform: TransformHandle| Ok(BindGraphicsChip::new_camera(gpu.clone(), transform)))?;
            module_table.set("newCamera", func)?;
        }
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, ()| Ok( 
                    gpu.borrow_mut().new_pass()
                )
            )?;
            module_table.set("newPass", func)?;
        }
        // Render state
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (r, g, b, a): (f32, f32, f32, f32)| Ok(
                    BindGraphicsChip::set_clear_color(
                        &mut gpu.borrow_mut(), 
                        &Vec4::new(r, g, b, a)
                    )
                )
            )?;
            module_table.set("setClearColor", func)?;
        }
        // Draw
        {
            let gpu = gpu.clone();
            let func = lua.create_function_mut(
                move |_, (x1, y1, x2, y2): (f32, f32, f32, f32)| Ok(
                    BindGraphicsChip::draw_line(
                        &mut gpu.borrow_mut(), 
                        &Vec2::new(x1, y1), 
                        &Vec2::new(x2, y2)
                    )
                )
            )?;
            module_table.set("line", func)?;
        }

        // add table to globals
        globals.set("graphics", module_table)?;

        Ok(())
    }
    
}