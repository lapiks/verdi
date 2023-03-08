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

    pub fn create_pass(&mut self) -> PassId {
        self.passes.insert(Pass::new())
    }

    pub fn get_pass_mut(&mut self, id: PassId) -> Option<&mut Pass> {
        self.passes.get_mut(id)
    }
}