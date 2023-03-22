use std::{cell::RefCell, rc::Rc};

use mlua::{UserData, UserDataMethods};
use verdi_math::prelude::Transform;

use crate::{
    render_cmds::DrawCmd, 
    mesh::{MeshHandle, MeshId}, 
    render_graph::RenderGraph, 
    model::ModelHandle, 
    render_state::RenderState, 
    camera::CameraHandle, 
    sprite::SpriteHandle
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

    pub fn add_draw_cmd(&mut self, mesh: MeshId, transform: Transform) {
        let cmd = DrawCmd {
            mesh,
            transform,
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
                    if let Some(cam_ref) = camera.database.borrow().assets.get_camera(camera.id) {
                        pass.render_state.view = cam_ref.transform.to_matrix();
                    }
                }
            })
        });
        methods.add_method_mut("drawModel", |_, pass, model: ModelHandle| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    if let Some(model_ref) = model.gpu.borrow().database.borrow().assets.get_model(model.id) {
                        for node in model_ref.get_nodes().iter() {
                            if let Some(mesh) = node.mesh {
                                pass.add_draw_cmd(mesh, node.transform);
                            }
                        }
                    }
                }
            })
        });
        methods.add_method_mut("drawMesh", |_, pass, (mesh, transform): (MeshHandle, Transform)| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.add_draw_cmd(mesh.id, transform);
                }
            })
        });
        methods.add_method_mut("drawSprite", |_, pass, sprite: SpriteHandle| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    // récupérer un quad à dessiner
                    //pass.add_draw_cmd(mesh.id, transform);
                }
            })
        });
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