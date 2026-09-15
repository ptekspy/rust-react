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

pub struct Button<C: NoClickable> {
    pub attributes: Attributes,
    pub children: Children<C>,
}

impl<C: NoClickable> Child for Button<C> {
    fn render(&self) -> String {
        format!(
            "<button{}>{}</button>",
            self.attributes.render(),
            self.children.render()
        )
    }
}

impl<C: NoClickable> Clickable for Button<C> {}

impl<C: NoClickable> Element for Button<C> {
    type Child = C;

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn children(&self) -> &Children<Self::Child> {
        &self.children
    }
}