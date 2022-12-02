use crate::app::App;

pub trait Command {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn execute(&self, app: &mut App);
}
