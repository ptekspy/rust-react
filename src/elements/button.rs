use crate::elements::children::Children;
use crate::traits::element::Element;
use crate::traits::{
    child::Child,
    clickable::{clickable::Clickable, no_clickable::NoClickable},
};

pub struct Button<C: NoClickable> {
    pub children: Children<C>,
}

impl<C: NoClickable> Child for Button<C> {
    fn render(&self) -> String {
        format!("<button>{}</button>", self.children.render())
    }
}

impl<C: NoClickable> Clickable for Button<C> {}
impl<C: NoClickable> Element for Button<C> {}
