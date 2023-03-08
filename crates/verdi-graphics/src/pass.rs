use mlua::{UserData, UserDataMethods};
use slotmap::new_key_type;

use crate::{render_cmds::RenderCmd, mesh::MeshHandle};

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
}

new_key_type! {
    pub struct PassId;
}

pub struct PassHandle {
    pub id: PassId,
}

impl UserData for PassHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("draw", |_, pass, mesh: MeshHandle| {
            Ok(())
        });
    }
}