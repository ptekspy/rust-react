use crate::elements::{
    attributes::Attributes,
    children::Children,
};
use crate::traits::child::Child;
use crate::traits::clickable::{
    clickable::Clickable,
    no_clickable::NoClickable,
};
use crate::traits::element::Element;

pub struct A<C: NoClickable> {
    pub attributes: Attributes,
    pub children: Children<C>,
}

impl<C: NoClickable> Child for A<C> {
    fn render(&self) -> String {
        format!(
            "<a{}>{}</a>",
            self.attributes.render(),
            self.children.render()
        )
    }
}

impl<C: NoClickable> Clickable for A<C> {}

impl<C: NoClickable> Element for A<C> {
    type Child = C;

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn children(&self) -> &Children<Self::Child> {
        &self.children
    }
}