use crate::globals::{
    empty::Empty,
    list::RenderList,
    more::More,
};

#[derive(Clone, Copy)]
pub struct AttributeRenderer;

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
}

impl RenderList<AttributeRenderer> for Attribute {
    fn render(&self, _renderer: AttributeRenderer) -> String {
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

impl<T> Attributes<T>
where
    T: RenderList<AttributeRenderer>,
{
    pub fn render(&self) -> String {
        self.items.render(AttributeRenderer)
    }
}