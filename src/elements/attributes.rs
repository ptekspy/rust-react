use crate::globals::{empty::Empty, more::More};

pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn render(&self) -> String {
        format!(" {}=\"{}\"", self.name, self.value)
    }
}

pub struct Attributes<T = Empty> {
    pub items: T,
}

impl Attributes<Empty> {
    pub fn new() -> Self {
        Self { items: Empty }
    }
}

impl<T> Attributes<T> {
    pub fn push(self, attribute: Attribute) -> Attributes<More<T, Attribute>> {
        Attributes {
            items: More {
                head: self.items,
                tail: attribute,
            },
        }
    }
}

impl<T: AttributeRenderer> Attributes<T> {
    pub fn render(&self) -> String {
        self.items.render()
    }
}

trait AttributeRenderer {
    fn render(&self) -> String;
}

impl AttributeRenderer for Empty {
    fn render(&self) -> String {
        String::new()
    }
}

impl AttributeRenderer for Attribute {
    fn render(&self) -> String {
        format!(" {}=\"{}\"", self.name, self.value)
    }
}

impl<H: AttributeRenderer, T: AttributeRenderer> AttributeRenderer for More<H, T> {
    fn render(&self) -> String {
        format!("{}{}", self.head.render(), self.tail.render())
    }
}