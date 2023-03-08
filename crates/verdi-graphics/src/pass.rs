use std::{cell::RefCell, rc::Rc};

use mlua::{UserData, UserDataMethods};
use slotmap::new_key_type;

use crate::{render_cmds::RenderCmd, mesh::MeshHandle, render_graph::RenderGraph};

pub struct CmdQueue {
    cmds: Vec<Box<dyn RenderCmd>>
}

impl CmdQueue {
    pub fn new() -> Self {
        Self {
            cmds: Vec::new(),
        }
    }
}

pub struct Pass {
    cmd_queue: CmdQueue,
}

impl Pass {
    pub fn new() -> Self {
        Self {
            cmd_queue: CmdQueue::new(),
        }
    }

    pub fn add_draw_cmd(&mut self) {

    }
}

new_key_type! {
    pub struct PassId;
}

pub struct PassHandle {
    pub graph: Rc<RefCell<RenderGraph>>,
    pub id: PassId,
}

impl UserData for PassHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("draw", |_, pass, mesh: MeshHandle| {
            Ok({
                if let Some(pass) = pass.graph.borrow_mut().get_pass_mut(pass.id) {
                    pass.add_draw_cmd();
                }
            })
        });
        methods.add_method_mut("enableFog", |_, pass, value: bool| {
            Ok(())
        });
        methods.add_method_mut("enableLighting", |_, pass, value: bool| {
            Ok(())
        });
        methods.add_method_mut("setFogStart", |_, pass, distance: f32| {
            Ok(())
        });
        methods.add_method_mut("setFogEnd", |_, pass, distance: f32| {
            Ok(())
        });
    }
}