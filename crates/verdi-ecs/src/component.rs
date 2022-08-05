
pub trait ComponentVec {
    // A reference to a type implementing this trait should be able to convert to Any  
    fn as_any(&self) -> &dyn std::any::Any;
    // A mut reference to a type implementing this trait should be able to convert to mut Any  
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    // Push a None value (aka an absent component) into the ComponentVec
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }
}
