use crate::elements::{
    attributes::Attributes,
    children::Children,
};
use crate::traits::child::Child;
use crate::traits::clickable::no_clickable::NoClickable;
use crate::traits::element::Element;

pub struct Div<C: Child> {
    pub attributes: Attributes,
    pub children: Children<C>,
}

impl<C: Child> Child for Div<C> {
    fn render(&self) -> String {
        format!(
            "<div{}>{}</div>",
            self.attributes.render(),
            self.children.render()
        )
    }
}

impl<C: NoClickable> NoClickable for Div<C> {}

impl<C: Child> Element for Div<C> {
    type Child = C;

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn children(&self) -> &Children<Self::Child> {
        &self.children
    }
}