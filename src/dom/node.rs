use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};

use super::EventTarget;

static NEXT_NODE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId(u64);

impl NodeId {
    fn next() -> Self {
        Self(NEXT_NODE_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    Element,
    Attribute,
    Text,
    CDataSection,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
}

impl NodeType {
    pub const fn value(self) -> u16 {
        match self {
            Self::Element => 1,
            Self::Attribute => 2,
            Self::Text => 3,
            Self::CDataSection => 4,
            Self::ProcessingInstruction => 7,
            Self::Comment => 8,
            Self::Document => 9,
            Self::DocumentType => 10,
            Self::DocumentFragment => 11,
        }
    }
}

pub struct Node {
    id: NodeId,
    event_target: EventTarget,
    node_type: NodeType,
    node_name: String,
}

impl Node {
    pub fn new(node_type: NodeType, node_name: impl Into<String>) -> Self {
        Self {
            id: NodeId::next(),
            event_target: EventTarget::new(),
            node_type,
            node_name: node_name.into(),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn event_target(&self) -> &EventTarget {
        &self.event_target
    }

    pub fn event_target_mut(&mut self) -> &mut EventTarget {
        &mut self.event_target
    }

    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn node_type_value(&self) -> u16 {
        self.node_type.value()
    }

    pub fn node_name(&self) -> &str {
        &self.node_name
    }
}

impl Deref for Node {
    type Target = EventTarget;

    fn deref(&self) -> &Self::Target {
        &self.event_target
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event_target
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeType};

    #[test]
    fn creates_node_with_type_and_name() {
        let node = Node::new(NodeType::Element, "div");

        assert_eq!(node.node_type(), NodeType::Element);
        assert_eq!(node.node_type_value(), 1);
        assert_eq!(node.node_name(), "div");
    }

    #[test]
    fn creates_text_node() {
        let node = Node::new(NodeType::Text, "#text");

        assert_eq!(node.node_type(), NodeType::Text);
        assert_eq!(node.node_type_value(), 3);
        assert_eq!(node.node_name(), "#text");
    }

    #[test]
    fn creates_unique_node_ids() {
        let first = Node::new(NodeType::Element, "div");
        let second = Node::new(NodeType::Element, "div");

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn node_id_exposes_numeric_value() {
        let node = Node::new(NodeType::Element, "div");

        assert!(node.id().value() > 0);
    }

    #[test]
    fn owns_event_target() {
        let node = Node::new(NodeType::Element, "div");

        let _event_target = node.event_target();
    }

    #[test]
    fn exposes_mutable_event_target() {
        let mut node = Node::new(NodeType::Element, "div");

        node.event_target_mut()
            .add_event_listener("click", |_| {});
    }

    #[test]
    fn dereferences_to_event_target() {
        let mut node = Node::new(NodeType::Element, "div");

        node.add_event_listener("click", |_| {});
    }
}