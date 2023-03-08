use slotmap::SlotMap;

use crate::pass::{PassId, Pass, PassHandle};

pub struct RenderGraph {
    passes: SlotMap<PassId, Pass>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self { 
            passes: SlotMap::default(),
        }
    }
}

impl RenderGraph {
    pub fn new() -> Self {
        RenderGraph::default()
    }

    pub fn create_pass(&mut self) -> PassHandle {
        PassHandle {
            id: self.passes.insert(Pass::new())
        }
    }
}