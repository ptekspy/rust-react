use crate::traits::child::Child;
use crate::traits::clickable::no_clickable::NoClickable;

pub struct Text {
    pub child: String,
}

impl Child for Text {
    fn render(&self) -> String {
        self.child.clone()
    }
}

impl NoClickable for Text {}