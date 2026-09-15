use crate::elements::{
    attributes::Attributes,
    children::Children,
};
use crate::traits::child::Child;

pub struct ElementData<C: Child> {
    pub attributes: Attributes,
    pub children: Children<C>,
}