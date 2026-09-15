use crate::{
    globals::list::RenderList,
    traits::child::Child,
};

#[derive(Clone, Copy)]
pub struct ChildRenderer;

pub fn render_child<T: Child>(child: &T) -> String {
    child.render()
}

impl<T: Child> RenderList<ChildRenderer> for T {
    fn render(&self, _renderer: ChildRenderer) -> String {
        render_child(self)
    }
}