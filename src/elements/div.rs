use crate::elements::children::Children;
use crate::traits::child::Child;
use crate::traits::clickable::no_clickable::NoClickable;

pub struct Div<C: Child> {
    pub children: Children<C>,
}

impl<C: Child> Child for Div<C> {
    fn render(&self) -> String {
        format!("<div>{}</div>", self.children.render())
    }
}

impl<C: NoClickable> NoClickable for Div<C> {}