use crate::traits::{child::Child, clickable::{clickable::Clickable, no_clickable::NoClickable}};

pub struct Button<C: NoClickable> {
    pub child: C,
}

impl<C: NoClickable> Child for Button<C> {
    fn render(&self) -> String {
        format!("<button>{}</button>", self.child.render())
    }
}

impl<C:NoClickable> Clickable for Button<C> {}