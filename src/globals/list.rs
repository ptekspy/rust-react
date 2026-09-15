use crate::globals::{empty::Empty, more::More};

pub trait RenderList {
    fn render(&self) -> String;
}

impl RenderList for Empty {
    fn render(&self) -> String {
        String::new()
    }
}

impl<H: RenderList, T: RenderList> RenderList for More<H, T> {
    fn render(&self) -> String {
        format!("{}{}", self.head.render(), self.tail.render())
    }
}