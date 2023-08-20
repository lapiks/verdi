use std::{rc::Rc, cell::RefCell, ops::Deref};

use slotmap::Key;
use verdi_database::{ResourceId, Resource, Handle, Assets};

pub type ShaderId = ResourceId;

pub struct Shader {
    src: String,
    pub id: ShaderId,
}

impl Resource for Shader {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Shader {
    pub fn new(src: String) -> Self {
        Self { 
            src,
            id: ShaderId::null(),
        }
    }

    pub fn get_source(&self) -> &str {
        self.src.as_str()
    }
}

pub struct ShaderHandle(Handle<Shader>);

impl Deref for ShaderHandle {
    type Target = Handle<Shader>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ShaderHandle {
    pub fn new(assets: Rc<RefCell<Assets>>, id: ShaderId) -> Self{
        Self(Handle::new(assets, id))
    }
}