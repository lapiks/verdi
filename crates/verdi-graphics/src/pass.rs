use std::{cell::RefCell, rc::Rc};

use mlua::{UserData, UserDataMethods};
use verdi_math::prelude::{TransformHandle, Transform};

use crate::{
    render_cmds::DrawCmd, 
    mesh::MeshHandle, 
    render_graph::RenderGraph, 
    model::{ModelHandle, Model}, 
    render_state::RenderState, 
    camera::{CameraHandle, Camera}, 
    sprite::{SpriteHandle, Sprite}
};

pub struct CmdQueue {
    cmds: Vec<DrawCmd>
}

impl CmdQueue {
    pub fn new() -> Self {
        Self {
            cmds: Vec::new(),
        }
    }

    pub fn push_cmd(&mut self, cmd: DrawCmd) {
        self.cmds.push(cmd);
    }
}

pub type PassId = u32;

pub struct Pass {
    cmd_queue: CmdQueue,
    pub render_state: RenderState,
}

impl Pass {
    pub fn new() -> Self {
        Self {
            cmd_queue: CmdQueue::new(),
            render_state: RenderState::new(),
        }
    }

    pub fn add_draw_cmd(&mut self, mesh: MeshHandle, transform: TransformHandle, perspective: bool) {
        let cmd = DrawCmd {
            mesh,
            transform,
            perspective,
        };

        self.cmd_queue.push_cmd(cmd);
    }

    pub fn get_cmds(&self) -> &Vec<DrawCmd> {
        &self.cmd_queue.cmds
    }
}

pub struct PassHandle {
    pub graph: Rc<RefCell<RenderGraph>>,
    pub id: PassId,
}

impl UserData for PassHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("submit", |_, pass, camera: CameraHandle| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    if let Some(cam_ref) = camera.get_assets().get_datas().get::<Camera>(camera.get_id()) {
                        if let Some(transform_ref) = cam_ref.transform.get_assets().get_datas().get::<Transform>(cam_ref.transform.get_id()) {
                            pass.render_state.view = transform_ref.to_matrix().inverse();
                        }   
                    }
                }
            })
        });
        methods.add_method_mut("drawModel", |_, pass, model: ModelHandle| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    if let Some(model_ref) = model.get_assets().get_datas().get::<Model>(model.get_id()) {
                        for node in model_ref.get_nodes().iter() {
                            if let Some(mesh) = &node.mesh {
                                pass.add_draw_cmd(
                                    mesh.clone(), 
                                    node.transform.clone(), 
                                    true
                                );
                            }
                        }
                    }
                }
            })
        });
        methods.add_method_mut("drawMesh", |_, pass, (mesh, transform): (MeshHandle, TransformHandle)| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.add_draw_cmd(
                        mesh, 
                        transform, 
                        true
                    );
                }
            })
        });
        // methods.add_method_mut("drawSprite", |_, pass, (sprite, transform): (SpriteHandle, TransformHandle)| {
        //     Ok({
        //         if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
        //             if let Some(sprite_ref) = sprite.get_assets().get::<Sprite>(sprite.get_id()) {
        //                 pass.add_draw_cmd(
        //                     sprite_ref.quad_id, 
        //                     transform.get_id(), 
        //                     false
        //                 );
        //             }
        //         }
        //     })
        // });
        methods.add_method_mut("enableLighting", |_, pass, value: bool| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.render_state.enable_lighting = value;
                }
            })
        });
        methods.add_method_mut("enableFog", |_, pass, value: bool| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.render_state.enable_fog = value;
                }
            })
        });
        methods.add_method_mut("setFogStart", |_, pass, value: f32| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.render_state.fog_start = value;
                }
            })
        });
        methods.add_method_mut("setFogEnd", |_, pass, value: f32| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.render_state.fog_end = value;
                }
            })
        });
    }
}