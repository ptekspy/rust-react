use crate::{
    globals::{
        empty::Empty,
        list::RenderList,
        more::More,
    },
    traits::{
        child::Child,
        clickable::no_clickable::NoClickable,
    },
};

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

impl<T: Child> Child for Children<T> {
    fn render(&self) -> String {
        self.items.render()
    }
}

impl<T: NoClickable> NoClickable for Children<T> {}