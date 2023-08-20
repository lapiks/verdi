use std::{ops::Deref, cell::RefCell, rc::Rc};

use glium::Display;
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    shader::{ShaderId, Shader}, 
    gpu_program::GpuProgram, gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset}
};

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

impl PrepareAsset for Program {
    fn prepare_rendering(&self, display: &Display, db: &Assets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        if let Some(vs) = db.get::<Shader>(self.vs) {
            if let Some(fs) = db.get::<Shader>(self.fs) {
                return Ok(
                    Box::new(
                        GpuProgram::new(display, vs, fs)
                    )
                );
            }
        }

        Err(GpuAssetError::PreparationFailed)
    }
}

pub struct ProgramHandle(Handle<Program>);

impl Deref for ProgramHandle {
    type Target = Handle<Program>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProgramHandle {
    pub fn new(assets: Rc<RefCell<Assets>>, id: ProgramId) -> Self{
        Self(Handle::new(assets, id))
    }
}