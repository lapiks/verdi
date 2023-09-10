use std::ops::Deref;

use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::shader::ShaderId;

pub type ProgramId = ResourceId;

pub struct Program {
    pub vs: ShaderId,
    pub fs: ShaderId,
    pub id: ProgramId,
}

impl Resource for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Program {
    pub fn new(vs: ShaderId, fs: ShaderId) -> Self {
        Self {
            vs,
            fs,
            id: ProgramId::null(),
        }
    }
}

pub struct ProgramHandle(Handle);

impl Deref for ProgramHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProgramHandle {
    pub fn new(assets: Assets, id: ProgramId) -> Self {
        ProgramHandle(assets.new_handle(id))
    }
}