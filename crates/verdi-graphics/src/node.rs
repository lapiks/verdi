use mlua::UserData;
use verdi_math::prelude::TransformHandle;

use crate::mesh::MeshHandle;


#[derive(Clone)]
pub struct Node {
    pub mesh: Option<MeshHandle>,
    pub transform: TransformHandle,
    pub children: Vec<Node>,
}

impl Node {
    // pub fn draw(&self, gpu: &mut GraphicsChip) {
    //     gpu.draw_node(&self);
    // }
}

impl UserData for Node {}

// impl Resource for Node {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

// pub struct NodeHandle(Handle<Node>);

// impl Deref for NodeHandle {
//     type Target = Handle<Node>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl NodeHandle {
//     pub fn new(assets: Rc<RefCell<Assets>>, id: NodeId) -> Self{
//         Self(Handle::new(assets, id))
//     }
// }

// impl UserData for NodeHandle {}