use crate::{pass::{Pass, PassId}, framebuffer::FramebufferHandle};

pub struct RenderGraph {
    passes: Vec<Pass>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self { 
            passes: Vec::default(),
        }
    }
}

impl RenderGraph {
    pub fn new() -> Self {
        RenderGraph::default()
    }

    pub fn create_pass(&mut self, framebuffer: FramebufferHandle) -> PassId {
        self.passes.push(Pass::new(framebuffer));
        (self.passes.len() - 1) as PassId
    }

    pub fn get_pass_mut(&mut self, id: PassId) -> Option<&mut Pass> {
        self.passes.get_mut(id as usize)
    }
    
    pub fn get_passes(&self) -> &Vec<Pass> {
        &self.passes
    }

    pub fn clear(&mut self) {
        self.passes.clear();
    }
}