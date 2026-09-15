use crate::traits::child::Child;
use crate::traits::clickable::no_clickable::NoClickable;

pub struct Text {
    pub string: String,
}

impl Child for Text {
    fn render(&self) -> String {
        self.string.clone()
    }
}

impl NoClickable for Text {}