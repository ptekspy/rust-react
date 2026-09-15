use crate::{globals::{empty::Empty, more::More}, traits::{
    child::Child,
    clickable::no_clickable::NoClickable,
}};

pub struct Children<T = Empty> {
    pub items: T,
}

impl Children<Empty> {
    pub fn new() -> Self {
        Self { items: Empty }
    }
}

impl<T> Children<T> {
    pub fn push<H>(self, head: H) -> Children<More<T, H>> {
        Children {
            items: More {
                head: self.items,
                tail: head,
            },
        }
    }
}

impl Child for Empty {
    fn render(&self) -> String {
        String::new()
    }
}

impl<H: Child, T: Child> Child for More<H, T> {
    fn render(&self) -> String {
        format!("{}{}", self.head.render(), self.tail.render())
    }
}

impl<T: Child> Child for Children<T> {
    fn render(&self) -> String {
        self.items.render()
    }
}

impl NoClickable for Empty {}

impl<H: NoClickable, T: NoClickable> NoClickable for More<H, T> {}

impl<T: NoClickable> NoClickable for Children<T> {}