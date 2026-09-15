use crate::globals::{
    empty::Empty,
    more::More,
};

pub trait RenderList<R> {
    fn render(&self, renderer: R) -> String;
}

impl<R> RenderList<R> for Empty {
    fn render(&self, _renderer: R) -> String {
        String::new()
    }
}

impl<R, H, T> RenderList<R> for More<H, T>
where
    R: Copy,
    H: RenderList<R>,
    T: RenderList<R>,
{
    fn render(&self, renderer: R) -> String {
        format!(
            "{}{}",
            self.head.render(renderer),
            self.tail.render(renderer),
        )
    }
}