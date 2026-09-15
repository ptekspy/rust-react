use crate::{
    elements::{
        attributes::Attributes,
        children::Children,
    },
    traits::child::Child,
};

pub trait Element: Child {
    type Child: Child;

    fn attributes(&self) -> &Attributes;
    fn children(&self) -> &Children<Self::Child>;
}