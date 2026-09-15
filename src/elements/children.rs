use crate::traits::child::Child;

pub struct Empty;

pub struct More<H, T> {
    pub head: H,
    pub tail: T,
}

pub struct Children<T = Empty> {
    pub items: T,
}

impl Children<Empty> {
    pub fn new() -> Self {
        Self { items: Empty }
    }
}

impl<T> Children<T> {
    pub fn push<H>(self, head: H) -> Children<More<H, T>> {
        Children {
            items: More {
                head,
                tail: self.items,
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