use crate::traits::child::Child;
use crate::traits::clickable::no_clickable::NoClickable;

pub struct Div<C: Child> {
    pub child: C,
}


impl<C: Child> Child for Div<C> {
    fn render(&self) -> String {
        format!("<div>{}</div>", self.child.render())
    }
}

impl<C: NoClickable> NoClickable for Div<C> {}