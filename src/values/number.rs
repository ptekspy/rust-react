use crate::traits::{child::Child, clickable::no_clickable::NoClickable};

pub struct Number {
    pub value: u32
}

impl Child for Number {
    fn render(&self) -> String {
        self.value.clone().to_string()
    }
}

impl NoClickable for Number {}