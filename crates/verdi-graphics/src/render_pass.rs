use crate::{  
    primitive::PrimitiveId, 
    transform::Transform,
};

pub struct RenderPass {
    // plutôt node qui contient primitive + transform
    pub primitive_id: PrimitiveId,
    pub transform: Transform
}
