use crate::elements::children::Children;
use crate::traits::{
    child::Child,
    clickable::{clickable::Clickable, no_clickable::NoClickable},
};

pub struct A<C: NoClickable> {
    pub children: Children<C>,
    pub href: String,
}

impl<C: NoClickable> Child for A<C> {
    fn render(&self) -> String {
        format!(
            "<a href=\"{}\">{}</a>",
            self.href,
            self.children.render()
        )
    }
}

impl<C: NoClickable> Clickable for A<C> {}